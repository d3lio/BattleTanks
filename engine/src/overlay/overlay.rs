extern crate gl;
extern crate cgmath;

use gliw::{
    Shader, ShaderType,
    Program, ProgramBuilder,
    Vao, Vbo, BufferType, BufferUsagePattern,
    AttribFloatFormat, UniformData,
};
use overlay::window::{WindowBase, BuildParams};

use self::cgmath::{Vector2, Vector3, Vector4, Vector, ApproxEq, Matrix4};
use std::cell::{RefCell, Cell};
use std::mem;
use std::ptr;
use std::slice;

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct VertexData {
    pos: Vec2,
    uv: Vec2,
    color: Vec4,
}

/// TODO: texture
pub struct OverlayBase {
    vao: Vao,
    vbo: Vbo,
    prog: Program,
    indices: Vec<u32>,      // TODO: maybe make the indices Vec<u16>

    arena: Vec<RefCell<WindowBase>>,

    pub should_update: RefCell<Vec<usize>>,
    pub should_reindex: Cell<bool>,
}

impl OverlayBase {
    pub fn new(width: u32, height: u32) -> OverlayBase {
        let vao = Vao::new();
        let vbo = Vbo::new(BufferType::Array);

        let vs = Shader::new(ShaderType::Vertex, VSHADER).unwrap();
        let fs = Shader::new(ShaderType::Fragment, FSHADER).unwrap();

        let prog = ProgramBuilder::new()
            .attach_vs(&vs)
            .attach_fs(&fs)
            .link()
            .unwrap();

        let vs_pos = prog.vert_attrib("vs_pos");
        vs_pos.data_float_format(&vao, &vbo, AttribFloatFormat::Float(2),
            mem::size_of::<VertexData>() as i32, ptr::null());
        vs_pos.enable(&vao);

        let vs_uv = prog.vert_attrib("vs_uv");
        vs_uv.data_float_format(&vao, &vbo, AttribFloatFormat::Float(2),
            mem::size_of::<VertexData>() as i32, mem::size_of::<Vec2>() as *const _);
        vs_uv.enable(&vao);

        let vs_color = prog.vert_attrib("vs_color");
        vs_color.data_float_format(&vao, &vbo, AttribFloatFormat::Float(4),
            mem::size_of::<VertexData>() as i32, (2 * mem::size_of::<Vec2>()) as *const _);
        vs_color.enable(&vao);

        let proj_mat = Matrix4::from_translation(cgmath::vec3(-1.0, 1.0, 0.0)) *
            Matrix4::from_nonuniform_scale(2.0 / width as f32, -2.0 / height as f32, 1.0);

        unsafe { prog.uniform("proj").value(UniformData::FloatMat(4, false, &mem::transmute::<_, [f32; 16]>(proj_mat))); }

        let mut root = WindowBase::new("", BuildParams {
            pos: cgmath::vec2(Vec3::zero(), Vec3::zero()),
            size: cgmath::vec2(cgmath::vec3(0.0, 0.0, width as f32), cgmath::vec3(0.0, 0.0, height as f32)),
            color: [Vec4::zero(); 4],
            texcoord: [Vec2::zero(); 4],
        });

        root.index = 0;
        root.vbo_beg = 0;
        root.vbo_end = 4;

        let mut ov = OverlayBase {
            vao: vao,
            vbo: vbo,
            prog: prog,
            indices: Vec::new(),

            arena: vec![RefCell::new(root)],

            should_update: RefCell::new(vec![0]),
            should_reindex: Cell::new(true),
        };

        // update vbo and indices
        ov.update();

        return ov;
    }

    pub fn update(&mut self) {
        if self.should_reindex.get() {
            let mut root = self.arena[0].borrow_mut();
            self.update_index(&mut root);
        }

        let should_update = self.should_update.borrow();

        if !should_update.is_empty() {
            let vbo_len = self.arena[0].borrow().vbo_end as usize;

            if self.should_reindex.get() {
                let mut vec: Vec<VertexData>;
                let vbo_data: &mut [VertexData];

                vec = vec![VertexData{pos: Vec2::zero(), uv: Vec2::zero(), color: Vec4::zero()}; vbo_len];
                vbo_data = vec.as_mut_slice();

                for &index in should_update.iter() {
                    self.update_subtree(&self.arena[index], vbo_data);
                }

                self.vbo.buffer_data(vbo_data, BufferUsagePattern::DynamicDraw);

                self.indices.truncate(vbo_len*6);
                for i in self.indices.len()/6 .. vbo_len {
                    self.indices.push(4*i as u32);
                    self.indices.push(4*i as u32 + 1);
                    self.indices.push(4*i as u32 + 2);

                    self.indices.push(4*i as u32);
                    self.indices.push(4*i as u32 + 2);
                    self.indices.push(4*i as u32 + 3);
                }
            }
            else {
                let vbo_data: &mut [VertexData];

                unsafe {
                    self.vbo.bind();
                    let ptr = gl::MapBuffer(self.vbo.buf_type() as u32, gl::WRITE_ONLY);
                    vbo_data = slice::from_raw_parts_mut(ptr as *mut VertexData, vbo_len);
                }

                for &index in should_update.iter() {
                    self.update_subtree(&self.arena[index], vbo_data);
                }

                unsafe { gl::UnmapBuffer(self.vbo.buf_type() as u32); }
            }

            mem::drop(should_update);
            let mut should_update = self.should_update.borrow_mut();

            should_update.clear();
            self.should_reindex.set(false);
        }
    }

    pub fn draw(&self) {
        self.vao.bind();
        self.prog.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, self.indices.as_ptr() as *const _);
        }
    }

    pub fn window_from_index(&self, index: usize) -> &RefCell<WindowBase> {
        return &self.arena[index];
    }

    pub fn make_window(&mut self, name: &str, data: BuildParams) -> usize {
        let next_index = self.arena.len();
        let mut window = WindowBase::new(name, data);
        window.index = next_index;

        self.arena.push(RefCell::new(window));
        return next_index;
    }

    fn update_index(&self, window: &mut WindowBase) {
        let mut prev_end = window.vbo_beg + 4;

        for &index in &window.children {
            let mut child = self.arena[index].borrow_mut();
            child.vbo_beg = prev_end;
            self.update_index(&mut child);
            prev_end = child.vbo_end;
        }

        window.vbo_end = prev_end;
    }

    fn update_subtree(&self, window_cell: &RefCell<WindowBase>, vbo_data: &mut [VertexData]) {
        let mut window = window_cell.borrow_mut();

        let new_pos: Vec2;
        let new_size: Vec2;

        if window.shown == false {
            new_pos = Vec2{x: -1.0, y: -1.0};
            new_size = Vec2{x: 0.0, y: 0.0};
        }
        else if let Some(parent_index) = window.parent {
            let parent = self.arena[parent_index].borrow();

            new_pos = Vec2 {
                x: parent.pos.x + Vec3::dot(window.creation_data.pos.x, cgmath::vec3(parent.size.x, parent.size.y, 1.0)),
                y: parent.pos.y + Vec3::dot(window.creation_data.pos.y, cgmath::vec3(parent.size.x, parent.size.y, 1.0)),
            };

            new_size = Vec2 {
                x: Vec3::dot(window.creation_data.size.x, cgmath::vec3(parent.size.x, parent.size.y, 1.0)),
                y: Vec3::dot(window.creation_data.size.y, cgmath::vec3(parent.size.x, parent.size.y, 1.0)),
            };
        }
        else {
            new_pos = Vec2{x: window.creation_data.pos.x.z, y: window.creation_data.pos.y.z};
            new_size = Vec2{x: window.creation_data.size.x.z, y: window.creation_data.size.y.z};
        }

        if !window.pos.approx_eq(&new_pos) || !window.size.approx_eq(&new_size) {
            window.pos = new_pos;
            window.size = new_size;

            vbo_data[window.vbo_beg as usize] = VertexData {
                pos: window.pos,
                uv: window.creation_data.texcoord[0],
                color: window.creation_data.color[0],
            };
            vbo_data[window.vbo_beg as usize + 1] = VertexData {
                pos: window.pos + cgmath::vec2(window.size.x, 0.0),
                uv: window.creation_data.texcoord[1],
                color: window.creation_data.color[1],
            };
            vbo_data[window.vbo_beg as usize + 2] = VertexData {
                pos: window.pos + window.size,
                uv: window.creation_data.texcoord[3],
                color: window.creation_data.color[3],
            };
            vbo_data[window.vbo_beg as usize + 3] = VertexData {
                pos: window.pos + cgmath::vec2(0.0, window.size.y),
                uv: window.creation_data.texcoord[2],
                color: window.creation_data.color[2],
            };

            mem::drop(window);
            let window = window_cell.borrow();

            for &index in &window.children {
                self.update_subtree(&self.arena[index], vbo_data);
            }
        }
    }
}

const VSHADER: &'static str = r#"
    #version 330 core

    uniform mat4 proj;
    in vec2 vs_pos;
    in vec2 vs_uv;
    in vec4 vs_color;
    out vec2 fs_uv;
    out vec4 fs_color;

    void main() {
        gl_Position = proj * vec4(vs_pos, 0.0, 1.0);
        fs_uv = vs_uv;
        fs_color = vs_color;
    }
"#;

const FSHADER: &'static str = r#"
    #version 330 core

    // uniform sampler2D tex;
    in vec2 fs_uv;
    in vec4 fs_color;
    out vec4 out_color;

    void main() {
        // out_color = texture(tex, fs_uv) + fs_color;
        out_color = fs_color;
    }
"#;

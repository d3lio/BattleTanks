extern crate gl;
extern crate cgmath;

use overlay::{Overlay, OverlayData, Window, WindowData, WindowParams};

use gliw::{
    Shader, ShaderType, ProgramBuilder,
    Vao, Vbo, BufferType, BufferUsagePattern,
    AttribFloatFormat, UniformData,
};

use self::cgmath::{Vector, Vector2, Matrix4};
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;
use std::ptr;

type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;
type Vec4 = cgmath::Vector4<f32>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct VertexData {
    pos: Vec2,
    uv: Vec2,
    color: Vec4,
}

impl Default for VertexData {
    fn default() -> Self {
        unsafe { mem::zeroed::<Self>() }
    }
}

impl OverlayData {
    pub fn new(width: u32, height: u32) -> OverlayData {
        let vao = Vao::new();
        let vbo = Vbo::new(BufferType::Array);

        // TODO: should I print a more helpfull error message
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

        let mut root = WindowData::new("", WindowParams {
            pos: Vector2::new(Vec3::zero(), Vec3::zero()),
            size: Vector2::new(Vec3::new(0.0, 0.0, width as f32), Vec3::new(0.0, 0.0, height as f32)),
            color: [Vec4::zero(); 4],
            texcoord: [Vec2::zero(); 4],
            shown: true,
        });
        root.vbo_beg = 0;

        let mut overlay = OverlayData {
            vao: vao,
            vbo: vbo,
            prog: prog,
            indices: Vec::new(),
            root: Window(Rc::new(Box::new(RefCell::new(root)))),
            should_reindex: true,
        };

        overlay.update();
        return overlay;
    }

    pub fn draw(&self) {
        self.vao.bind();
        self.prog.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, self.indices.as_ptr() as *const _);
        }
    }

    pub fn update(&mut self) {
        if !self.should_reindex {
            return;
        }

        let len = reindex(self.root.clone()) as usize;

        // update indices array
        self.indices.truncate(6 * len);
        for i in self.indices.len()/6 .. len {
            self.indices.push(4*i as u32);
            self.indices.push(4*i as u32 + 1);
            self.indices.push(4*i as u32 + 2);

            self.indices.push(4*i as u32);
            self.indices.push(4*i as u32 + 2);
            self.indices.push(4*i as u32 + 3);
        }

        self.should_reindex = false;

        unsafe {
            self.vbo.bind();
            gl::BufferData(self.vbo.buf_type() as u32, (4 * len * mem::size_of::<VertexData>()) as isize,
                ptr::null(), BufferUsagePattern::DynamicDraw as u32);
        }

        self.update_subtree(self.root.clone());

        fn reindex(window: Window) -> isize {
            let mut prev_end;
            {
                let window_ref = window.0.borrow();
                prev_end = window_ref.vbo_beg + 1;

                for child in &window_ref.children {
                    child.0.borrow_mut().vbo_beg = prev_end;
                    prev_end = reindex(child.clone());
                }
            }

            window.0.borrow_mut().vbo_end = prev_end;
            return prev_end;
        }
    }

    pub fn update_subtree(&self, window: Window) {
        if self.should_reindex {
            return;
        }

        let len;
        let offset;
        {
            let window_ref = window.0.borrow();
            len = (window_ref.vbo_end - window_ref.vbo_beg) as usize;
            offset = window_ref.vbo_beg as usize;
        }

        let mut vbo_data = vec![VertexData::default(); 4 * len];
        helper(window, &mut vbo_data, 4 * offset);

        unsafe {
            self.vbo.bind();
            gl::BufferSubData(self.vbo.buf_type() as u32, (4 * offset * mem::size_of::<VertexData>()) as isize,
                (4 * len * mem::size_of::<VertexData>()) as isize, vbo_data.as_ptr() as *const _);
        }

        fn helper(window: Window, vbo_data: &mut Vec<VertexData>, offset: usize) {
            let new_pos: Vec2;
            let new_size: Vec2;
            {
                let window_ref = window.0.borrow();

                if window_ref.creation_data.shown == false {
                    new_pos = Vec2{x: -1.0, y: -1.0};
                    new_size = Vec2{x: 0.0, y: 0.0};
                }
                else if let Some(parent) = window_ref.parent.upgrade() {
                    let parent = parent.borrow();

                    new_pos = Vec2 {
                        x: parent.pos.x + Vec3::dot(window_ref.creation_data.pos.x, cgmath::vec3(parent.size.x, parent.size.y, 1.0)),
                        y: parent.pos.y + Vec3::dot(window_ref.creation_data.pos.y, cgmath::vec3(parent.size.x, parent.size.y, 1.0)),
                    };

                    new_size = Vec2 {
                        x: Vec3::dot(window_ref.creation_data.size.x, cgmath::vec3(parent.size.x, parent.size.y, 1.0)),
                        y: Vec3::dot(window_ref.creation_data.size.y, cgmath::vec3(parent.size.x, parent.size.y, 1.0)),
                    };
                }
                else {
                    new_pos = Vec2{x: window_ref.creation_data.pos.x.z, y: window_ref.creation_data.pos.y.z};
                    new_size = Vec2{x: window_ref.creation_data.size.x.z, y: window_ref.creation_data.size.y.z};
                }
            }

            {
                let mut window_mut = window.0.borrow_mut();
                window_mut.pos = new_pos;
                window_mut.size = new_size;
            }

            let window_ref = window.0.borrow();

            vbo_data[4 * window_ref.vbo_beg as usize - offset] = VertexData {
                pos: window_ref.pos,
                uv: window_ref.creation_data.texcoord[0],
                color: window_ref.creation_data.color[0],
            };
            vbo_data[4 * window_ref.vbo_beg as usize + 1 - offset] = VertexData {
                pos: window_ref.pos + cgmath::vec2(window_ref.size.x, 0.0),
                uv: window_ref.creation_data.texcoord[1],
                color: window_ref.creation_data.color[1],
            };
            vbo_data[4 * window_ref.vbo_beg as usize + 2 - offset] = VertexData {
                pos: window_ref.pos + window_ref.size,
                uv: window_ref.creation_data.texcoord[3],
                color: window_ref.creation_data.color[3],
            };
            vbo_data[4 * window_ref.vbo_beg as usize + 3 - offset] = VertexData {
                pos: window_ref.pos + cgmath::vec2(0.0, window_ref.size.y),
                uv: window_ref.creation_data.texcoord[2],
                color: window_ref.creation_data.color[2],
            };

            for child in &window_ref.children {
                helper(child.clone(), vbo_data, offset);
            }
        }
    }
}

impl Drop for OverlayData {
    fn drop(&mut self) {
        // let root = self.root.0.borrow();

        // TODO: this causes the rc to be borrowed twice
        // instead we could just update the overlay pointer

        // for window in &root.children {
        //     window.detach();
        // }
    }
}

impl Overlay {
    /// Create a new Overlay.
    #[inline]
    pub fn new(width: u32, height: u32) -> Overlay {
        Overlay(RefCell::new(OverlayData::new(width, height)))
    }

    /// Render the overlay windows.
    ///
    /// In order to render correctly depth testing must be disabled and alpha blending enabled.
    #[inline]
    pub fn draw(&self) {
        let mut ovl = self.0.borrow_mut();
        ovl.update();
        ovl.draw();
    }

    /// Get handle to the root window of the overlay.
    #[inline]
    pub fn root(&self) -> Window {
        self.0.borrow().root.clone()
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

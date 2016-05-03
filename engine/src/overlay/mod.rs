//! A 2D Overlay
//!
//! An overlay object represents something that is drawn on a portion of the screen after the rest of the
//! scene has been rendered.
//!
//! It uses its own relative coordinate system, where the center (0, 0) is the leftmost uppermost point,
//! the positive X axis points right, the positive Y axis points down and one unit is one pixel.
//!
//! An `Overlay` is composed of several `Window` objects, attached together in a hierarchal structure.
//! Each overlay has a root window which encompasses the whole overlay area (which usually is the whole vieport).
//! Only windows attached to the root are rendered.
//!
//! Each window has a name, which is a string not containing the `.` character. Several window names, separated
//! by a `.` character represents a path in the hierarchy three. Paths are used for identification, so while it
//! is possible to have two windows with the same name, two different paths cannot have the same string representation.
//!
//! Each `Window` represents a rectangular area of the screen that is drawn over.
//! Windows are represented by the four vertices of the rectangle:
//!
//!   * vertex 0 - upper left
//!   * vertex 1 - upper right
//!   * vertex 2 - bottom left
//!   * vertex 3 - bottom right
//!
//! The order of rendering is dependent on the order in which windows are attached to each other. First is
//! rendered the subtree of the first attached child, then the subtree of the next attached child, etc,
//! continuing recursively using the same algorithm.
//! In other words it is a pre-order traversal of the hierarchy tree.

mod window;
pub mod single_thread;

extern crate gl;
extern crate cgmath;

use gliw::{
    Shader, ShaderType,
    Program, ProgramBuilder,
    Vao, Vbo, BufferType, BufferUsagePattern,
    AttribFloatFormat, UniformData,
};
use overlay::window::{Window, WindowParams};

use self::cgmath::{Vector2, Matrix4, Vector};
use std::cell::RefCell;
use std::rc::Rc;
use std::mem;
use std::ptr;
use std::default::Default;

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

// TODO: texture
pub struct Overlay {
    vao: Vao,
    vbo: Vbo,
    prog: Program,
    indices: Vec<u32>,      // TODO: maybe make the indices Vec<u16>

    root: Rc<Box<RefCell<Window>>>,

    should_reindex: bool,
}

impl Overlay {
    fn new(width: u32, height: u32) -> Overlay {
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

        let mut root = Window::new("", WindowParams {
            pos: Vector2::new(Vec3::zero(), Vec3::zero()),
            size: Vector2::new(Vec3::new(0.0, 0.0, width as f32), Vec3::new(0.0, 0.0, height as f32)),
            color: [Vec4::zero(); 4],
            texcoord: [Vec2::zero(); 4],
        });
        root.vbo_beg = 0;

        let mut overlay = Overlay {
            vao: vao,
            vbo: vbo,
            prog: prog,
            indices: Vec::new(),
            root: Rc::new(Box::new(RefCell::new(root))),
            should_reindex: true,
        };

        overlay.update();
        return overlay;
    }

    fn draw(&self) {
        self.vao.bind();
        self.prog.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, self.indices.as_ptr() as *const _);
        }
    }

    fn update(&mut self) {
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
            gl::BufferData(self.vbo.buf_type() as u32, (len * mem::size_of::<VertexData>()) as isize,
                ptr::null(), BufferUsagePattern::DynamicDraw as u32);
        }

        self.update_subtree(self.root.clone());

        fn reindex(window: Rc<Box<RefCell<Window>>>) -> isize {
            let mut prev_end;
            {
                let window = window.borrow();
                prev_end = window.vbo_beg + 1;

                for child in &window.children {
                    child.borrow_mut().vbo_beg = prev_end;
                    prev_end = reindex(child.clone());
                }
            }

            window.borrow_mut().vbo_end = prev_end;
            return prev_end;
        }
    }

    fn update_subtree(&self, window: Rc<Box<RefCell<Window>>>) {
        if self.should_reindex {
            return;
        }

        let len;
        let offset;
        {
            let win = window.borrow();
            len = (win.vbo_end - win.vbo_beg) as usize;
            offset = win.vbo_beg as usize;
        }

        let mut vbo_data = vec![VertexData::default(); 4 * len];
        helper(window, &mut vbo_data);

        unsafe {
            self.vbo.bind();
            gl::BufferSubData(self.vbo.buf_type() as u32, (offset * mem::size_of::<VertexData>()) as isize,
                (len * mem::size_of::<VertexData>()) as isize, vbo_data.as_ptr() as *const _);
        }

        fn helper(window: Rc<Box<RefCell<Window>>>, vbo_data: &mut Vec<VertexData>) {
            let new_pos: Vec2;
            let new_size: Vec2;
            {
                let window = window.borrow();

                if window.shown == false {
                    new_pos = Vec2{x: -1.0, y: -1.0};
                    new_size = Vec2{x: 0.0, y: 0.0};
                }
                else if let Some(parent) = unwrap_weak(&window.parent) {
                    let parent = parent.borrow();

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
            }

            {
                let mut window = window.borrow_mut();
                window.pos = new_pos;
                window.size = new_size;
            }

            let window = window.borrow();

            vbo_data[4 * window.vbo_beg as usize] = VertexData {
                pos: window.pos,
                uv: window.creation_data.texcoord[0],
                color: window.creation_data.color[0],
            };
            vbo_data[4 * window.vbo_beg as usize + 1] = VertexData {
                pos: window.pos + cgmath::vec2(window.size.x, 0.0),
                uv: window.creation_data.texcoord[1],
                color: window.creation_data.color[1],
            };
            vbo_data[4 * window.vbo_beg as usize + 2] = VertexData {
                pos: window.pos + window.size,
                uv: window.creation_data.texcoord[3],
                color: window.creation_data.color[3],
            };
            vbo_data[4 * window.vbo_beg as usize + 3] = VertexData {
                pos: window.pos + cgmath::vec2(0.0, window.size.y),
                uv: window.creation_data.texcoord[2],
                color: window.creation_data.color[2],
            };

            for child in &window.children {
                helper(child.clone(), vbo_data);
            }
        }
    }
}

// Temporary function until `downgraded_weak` is stabilized
use std::rc::Weak;
fn unwrap_weak<T> (val: &Option<Weak<T>>) -> Option<Rc<T>> {
    match *val {
        Some(ref weak) => weak.upgrade(),
        None => None,
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

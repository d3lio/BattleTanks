extern crate engine;
extern crate cgmath;
extern crate glfw;
extern crate gl;

use engine::gliw::{
    Vao,
    Vbo, BufferType, BufferUsagePattern,
    Shader, ShaderType,
    Program, ProgramBuilder
};
use cgmath::{
    Matrix, SquareMatrix, Matrix4,
    Angle, Deg,
    Vector3, Point3,
};
use glfw::{Action, Context, Key};

use std::ptr;
use std::ffi::CString;

static VERTEX_DATA: [f32; 9] = [
    -1.0, -1.0, 0.0,
     1.0, -1.0, 0.0,
     0.0,  1.0, 0.0,
];

static VS_SRC: &'static str = r#"
    #version 330 core

    layout (location = 0) in vec3 position;
    uniform mat4 mvp;

    void main() {
       gl_Position = mvp * vec4(position, 1.0);
    }
"#;

static FS_SRC: &'static str = r#"
    #version 330 core

    out vec3 color;

    void main() {
       color = vec3(1.0, 1.0, 1.0);
    }
"#;

struct SimpleEntity {
    vao: Vao,
    vbos: Vec<Vbo>,
    program: Program,
    mvp_matrix: Matrix4<f32>,
    mvp_location: i32
}

impl /*Entity for*/ SimpleEntity {
    fn new() -> SimpleEntity {
        let vs = Shader::new(ShaderType::Vertex, VS_SRC).unwrap();
        let fs = Shader::new(ShaderType::Fragment, FS_SRC).unwrap();
        let program = ProgramBuilder::new()
            .attach_vs(&vs)
            .attach_fs(&fs)
            .link()
            .unwrap();

        let mut obj = SimpleEntity {
            vao: Vao::new(),
            vbos: Vec::<Vbo>::new(),
            program: program,
            mvp_matrix: Matrix4::<f32>::identity(),
            mvp_location: 0
        };

        obj.vao.bind();
        obj.vbos.push(
            Vbo::from_data(
                &VERTEX_DATA,
                BufferType::Array,
                BufferUsagePattern::StaticDraw));

        let model_matrix = Matrix4::from_translation(
            Vector3::<f32>::new(0.0, 0.0, 0.0));

        let view_matrix = Matrix4::look_at(
            Point3::<f32>::new(4.0, 3.0, 3.0),
            Point3::<f32>::new(0.0, 0.0, 0.0),
            Vector3::<f32>::new(0.0, 1.0, 0.0));

        let proj_matrix = cgmath::perspective(
            Deg::new(45.0), 4.0/3.0, 0.01, 100.0);

        obj.mvp_matrix = proj_matrix * view_matrix * model_matrix;

        unsafe {
            obj.mvp_location = gl::GetUniformLocation(obj.program.handle(), CString::new("mvp").unwrap().as_ptr());
        }

        return obj;
    }

    fn draw_self(&self) {
        self.program.bind();

        unsafe {
            gl::UniformMatrix4fv(self.mvp_location as i32, 1, gl::FALSE, self.mvp_matrix.as_ptr());

            let attrib_location = 0;

            gl::EnableVertexAttribArray(attrib_location);
            self.vbos[0].bind();
            gl::VertexAttribPointer(attrib_location, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::DisableVertexAttribArray(attrib_location);
        }
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Samples(4));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) =
        glfw.create_window(
            800, 600,
            "It's alive!",
            glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    glfw.set_swap_interval(1);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe { gl::ClearColor(0.0, 0.0, 0.4, 0.0); }

    let entity = SimpleEntity::new();

    while !window.should_close() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

        entity.draw_self();

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

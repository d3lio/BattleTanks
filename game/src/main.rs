extern crate engine;
extern crate cgmath;
extern crate glfw;
extern crate gl;

use engine::gliw::{
    Vao,
    Vbo, BufferType, BufferUsagePattern,
    Shader, ShaderType,
    Program, ProgramBuilder,
    Uniform, UniformData,
    VertexAttrib, AttribFloatFormat,
    Texture, TextureBuilder2D, ImageType, TextureCoordWrap, TextureFilter
};
use cgmath::{
    Matrix4,
    Angle, Deg,
    Vector3, Point3,
};
use glfw::{Action, Context, Key};

use std::ptr;
use std::mem;

static VERTEX_DATA: [f32; 18] = [
    -1.0,  1.0, 0.0,
    -1.0, -1.0, 0.0,
     1.0, -1.0, 0.0,

    -1.0,  1.0, 0.0,
     1.0, -1.0, 0.0,
     1.0,  1.0, 0.0,
];

static COLOR_DATA: [f32; 12] = [
    8.0, 8.0,
    8.0, 0.0,
    0.0, 0.0,

    8.0, 8.0,
    0.0, 0.0,
    0.0, 8.0,
];

#[allow(dead_code)]
struct SimpleEntity {
    vao: Vao,
    vbos: Vec<Vbo>,
    program: Program,
    mvp_location: Uniform,
    mvp_matrix: Matrix4<f32>,
    attribs: Vec<VertexAttrib>,
    tex: Texture,
}

impl SimpleEntity {
    fn new() -> SimpleEntity {
        let vs = Shader::from_file(ShaderType::Vertex, "resources/shaders/vs.glsl").unwrap();
        let fs = Shader::from_file(ShaderType::Fragment, "resources/shaders/fs.glsl").unwrap();
        let program = ProgramBuilder::new()
            .attach_vs(&vs)
            .attach_fs(&fs)
            .link()
            .unwrap();

        let vao = Vao::new();
        let mut vbos = Vec::<Vbo>::new();

        vao.bind();
        vbos.push(
            Vbo::from_data(
                &VERTEX_DATA,
                BufferType::Array,
                BufferUsagePattern::StaticDraw));
        vbos.push(
            Vbo::from_data(
                &COLOR_DATA,
                BufferType::Array,
                BufferUsagePattern::StaticDraw));

        let model_matrix = Matrix4::from_translation(
            Vector3::<f32>::new(0.0, 0.0, 0.0));

        let view_matrix = Matrix4::look_at(
            Point3::<f32>::new(2.0, 1.5, 3.0),
            Point3::<f32>::new(0.0, 0.0, 0.0),
            Vector3::<f32>::new(0.0, 1.0, 0.0));

        let proj_matrix = cgmath::perspective(
            Deg::new(45.0), 4.0/3.0, 0.01, 100.0);

        let mvp_matrix = proj_matrix * view_matrix * model_matrix;
        let mvp_location = program.get_uniform_loc("mvp");

        let mut attribs = Vec::<VertexAttrib>::new();
        attribs.push(VertexAttrib::new(0));
        attribs[0].data_float_format(&vao, &vbos[0], AttribFloatFormat::Float(3), 0, ptr::null());
        attribs.push(VertexAttrib::new(1));
        attribs[1].data_float_format(&vao, &vbos[1], AttribFloatFormat::Float(2), 0, ptr::null());


        let tex = TextureBuilder2D::new()
            .source("resources/textures/banana.bmp", ImageType::Bmp)
            .wrap(TextureCoordWrap::Repeat, TextureCoordWrap::Repeat)
            .filter(TextureFilter::LinearMipmapLinear, TextureFilter::Linear)
            .gen_mipmap()
            .load()
            .unwrap();

        tex.pass_to(&program, "tex", 0);

        return SimpleEntity {
            vao: vao,
            vbos: vbos,
            program: program,
            mvp_matrix: mvp_matrix,
            mvp_location: mvp_location,
            attribs: attribs,
            tex: tex
        };
    }

    fn draw_self(&self) {
        self.vao.bind();
        self.program.bind();

        unsafe {
            self.mvp_location.value(
                &self.program,
                UniformData::FloatMat(4, false, &mem::transmute::<Matrix4<f32>, [f32; 16]>(self.mvp_matrix) )
            );
        }

        for attrib in &self.attribs {
            (*attrib).enable(&self.vao);
        }

        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 6); }

        for attrib in &self.attribs {
            (*attrib).disable(&self.vao);
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
            "Going bananas!",
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

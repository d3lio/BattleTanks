extern crate engine;
extern crate cgmath;
extern crate glfw;
extern crate gl;

#[allow(unused_imports)]
use engine::gliw::{
    Buffer, BufferType, BufferUsagePattern,
    Gliw, DepthFunction,
    Program, ProgramBuilder,
    Shader, ShaderType,
    Texture, TextureBuilder2D, ImageType, TextureCoordWrap, TextureFilter,
    Uniform, UniformData,
    Vao,
    VertexAttrib, AttribFloatFormat,
};

use engine::core::{Entity, Camera, Renderable, Scene, Cuboid, Color};

use cgmath::{Point3, Vector3, Matrix4};

use glfw::{Action, Context, Key};

use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;
use std::mem;

static VERTEX_DATA: [f32; 18] = [
    -0.5, 1.0, 0.501,
    -0.5, 0.0, 0.501,
     0.5, 0.0, 0.501,

    -0.5, 1.0, 0.501,
     0.5, 0.0, 0.501,
     0.5, 1.0, 0.501,
];

static COLOR_DATA: [f32; 12] = [
    4.0, 4.0,
    4.0, 0.0,
    0.0, 0.0,

    4.0, 4.0,
    0.0, 0.0,
    0.0, 4.0,
];

#[allow(dead_code)]
struct SimpleEntity {
    vao: Vao,
    vbos: Vec<Buffer>,
    program: Rc<Program>,
    model_matrix: Matrix4<f32>,
    attribs: Vec<VertexAttrib>,
    tex: Texture,
}

impl SimpleEntity {
    fn new(program: Rc<Program>) -> SimpleEntity {
        let vao = Vao::new();
        let mut vbos = Vec::<Buffer>::new();

        vao.bind();
        vbos.push(
            Buffer::from_data(
                &VERTEX_DATA,
                BufferType::Array,
                BufferUsagePattern::StaticDraw));
        vbos.push(
            Buffer::from_data(
                &COLOR_DATA,
                BufferType::Array,
                BufferUsagePattern::StaticDraw));

        let model_matrix = Matrix4::from_translation(
            Vector3::<f32>::new(0.0, 0.0, 0.0));

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
            model_matrix: model_matrix,
            attribs: attribs,
            tex: tex
        };
    }
}

impl Renderable for SimpleEntity {
    fn draw(&self, camera: &Camera) {
        self.vao.bind();
        self.program.bind();

        let mvp_matrix = camera.vp_matrix() * self.model_matrix;

        unsafe {
            self.program.uniform("mvp").value(UniformData::FloatMat(4, false,
                &mem::transmute::<Matrix4<f32>, [f32; 16]>(mvp_matrix)));
        }

        for attrib in &self.attribs {
            attrib.enable(&self.vao);
        }

        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 6); }

        for attrib in &self.attribs {
            attrib.disable(&self.vao);
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
            "Cuboid bananas!",
            glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    glfw.set_swap_interval(1);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Done initializing the window

    Gliw::enable(gl::DEPTH_TEST);
    Gliw::depth_func(DepthFunction::Less);
    Gliw::enable(gl::CULL_FACE);
    Gliw::clear_color(0.0, 0.0, 0.4, 0.0);

    let mut camera = Camera::new();
    camera.look_at(
        Point3::<f32>::new(4.0, 3.0, 6.0),
        Point3::<f32>::new(0.0, 0.0, 0.0),
        Vector3::<f32>::new(0.0, 1.0, 0.0));
    camera.perspective(45.0, 4.0/3.0, 0.01, 100.0);

    let vs = Shader::from_file(ShaderType::Vertex, "resources/shaders/vs.glsl").unwrap();
    let fs = Shader::from_file(ShaderType::Fragment, "resources/shaders/fs.glsl").unwrap();
    let program = ProgramBuilder::new()
        .attach_vs(&vs)
        .attach_fs(&fs)
        .link()
        .unwrap();
    let entity = Rc::new(RefCell::new(SimpleEntity::new(Rc::new(program))));

    let cuboid1_rc = Rc::new(RefCell::new(Cuboid::new(
        Point3::new(0.0, 0.5, 0.0),
        Vector3::new(1.0, 1.0, 1.0),
        Color::from_rgba(51, 102, 255, 255))));

    let cuboid2_rc = Rc::new(RefCell::new(Cuboid::new(
        Point3::new(1.375, 0.5, 1.0),
        Vector3::new(1.75, 1.0, 1.0),
        Color::from_rgba(153, 153, 255, 255))));

    let cuboid3_rc = Rc::new(RefCell::new(Cuboid::new(
        Point3::new(1.375, 0.875, -0.375),
        Vector3::new(1.0, 1.0, 1.0),
        Color::from_rgba(255, 0, 102, 255))));

    let cuboid4_rc = Rc::new(RefCell::new(Cuboid::new(
        Point3::new(-2.0, 0.5, 1.0),
        Vector3::new(1.0, 1.0, 1.0),
        Color::from_rgba(51, 204, 51, 255))));

    let cuboid5_rc = Rc::new(RefCell::new(Cuboid::new(
        Point3::new(-1.0, 0.5, -1.0),
        Vector3::new(1.0, 1.0, 1.0),
        Color::from_rgba(255, 102, 0, 255))));
    cuboid5_rc.borrow_mut().look_at(
        Vector3::new(-2.0, 0.0, -2.0),
        Vector3::new(0.0, 1.0, 0.0));

    let cuboid6_rc = Rc::new(RefCell::new(Cuboid::new(
        Point3::new(-2.5, 0.25, -0.5),
        Vector3::new(0.5, 0.5, 0.5),
        Color::from_rgba(255, 204, 0, 255))));

    let platform = Rc::new(RefCell::new(Cuboid::new(
        Point3::new(0.0, -0.05, 0.0),
        Vector3::new(7.0, 0.1, 4.0),
        Color::from_rgba(153, 51, 255, 255))));

    let mut scene = Scene::new(camera);
    scene.add(Rc::downgrade(&platform));
    scene.add(Rc::downgrade(&entity));
    scene.add(Rc::downgrade(&cuboid1_rc));
    scene.add(Rc::downgrade(&cuboid2_rc));
    scene.add(Rc::downgrade(&cuboid3_rc));
    scene.add(Rc::downgrade(&cuboid4_rc));
    scene.add(Rc::downgrade(&cuboid5_rc));
    scene.add(Rc::downgrade(&cuboid6_rc));

    let animation_speed = 2.0;
    let camera_speed = 0.5;
    let cuboid3_scale = cuboid3_rc.borrow().scale();
    let cuboid4_pos_x = cuboid4_rc.borrow().position().x;

    while !window.should_close() {
        Gliw::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        cuboid3_rc.borrow_mut().scale_to(cuboid3_scale +
            (f64::sin(glfw.get_time() * animation_speed) as f32) * 0.75);

        cuboid4_rc.borrow_mut().center().x = cuboid4_pos_x +
            f64::sin(glfw.get_time() * animation_speed) as f32;

        cuboid6_rc.borrow_mut().look_at(
            Vector3::new(
                f64::cos(glfw.get_time() * animation_speed) as f32,
                0.0,
                f64::sin(glfw.get_time() * animation_speed) as f32),
            Vector3::new(0.0, 1.0, 0.0));

        scene.camera_mut().look_at(
            Point3::<f32>::new(
                4.0 * f64::cos(glfw.get_time() * camera_speed) as f32,
                3.0 * (f64::cos(glfw.get_time() * camera_speed) * 0.5 + 1.0) as f32,
                6.0 * f64::sin(glfw.get_time() * camera_speed) as f32),
            Point3::<f32>::new(0.0, 0.0, 0.0),
            Vector3::<f32>::new(0.0, 1.0, 0.0));

        scene.draw();

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

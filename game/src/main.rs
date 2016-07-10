#[macro_use(wrap, events, key_mask)]
extern crate engine;
extern crate cgmath;
extern crate glfw;
extern crate gl;

use engine::gliw::{Gliw, DepthFunction, ProgramBuilder, Shader, ShaderType};
use engine::core::{Camera, Renderable, Scene, Composition, Cuboid, Color, Event, Data};
use engine::core::input::{Manager, KeyListener};
use engine::overlay::{Overlay, Window, WindowParams};

use cgmath::{Vector2, Vector3, Vector4, Point3, VectorSpace};
use glfw::{Action, Context, Key};
use std::ffi::CStr;

mod simple_plain;
mod simple_component;

use self::simple_plain::SimplePlain;
use self::simple_component::AntiClockwiseRotation;

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

    unsafe {
        println!("GL Version: {:?}", CStr::from_ptr(gl::GetString(gl::VERSION) as *const _));
        println!("GL Renderer: {:?}", CStr::from_ptr(gl::GetString(gl::RENDERER) as *const _));
    }

    Gliw::clear_color(0.0, 0.0, 0.4, 0.0);
    Gliw::enable(gl::BLEND);
    unsafe { gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); }

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

    let entity = wrap!(SimplePlain::new(program.clone()));

    let cuboid1 = wrap!(Cuboid::new(
        Point3::new(0.0, 0.5, 0.0),
        Vector3::new(1.0, 1.0, 1.0),
        Color::from_rgba(51, 102, 255, 255)));

    let cuboid2 = wrap!(Cuboid::new(
        Point3::new(1.375, 0.5, 1.0),
        Vector3::new(1.75, 1.0, 1.0),
        Color::from_rgba(153, 153, 255, 255)));

    let cuboid3 = wrap!(Cuboid::new(
        Point3::new(1.375, 0.875, -0.375),
        Vector3::new(1.0, 1.0, 1.0),
        Color::from_rgba(255, 0, 102, 255)));

    let cuboid4 = wrap!(Composition::new(Cuboid::new(
        Point3::new(-2.0, 0.5, 1.0),
        Vector3::new(1.0, 1.0, 1.0),
        Color::from_rgba(51, 204, 51, 255))));

    let cuboid4_child_comp = wrap!(Composition::new(Cuboid::new(
        Point3::new(0.0, 0.75, 0.0),
        Vector3::new(0.5, 0.5, 0.5),
        Color::from_rgba(153, 204, 0, 255))));

    let cuboid4_child_comp_child = wrap!(Cuboid::new(
        Point3::new(-1.0, 0.75, 0.0),
        Vector3::new(0.5, 0.5, 0.5),
        Color::from_rgba(0, 204, 102, 255)));

    cuboid4.borrow_mut().attach(Scene::node(&cuboid4_child_comp));
    cuboid4_child_comp.borrow_mut().attach(Scene::node(&cuboid4_child_comp_child));

    let cuboid5 = wrap!(Cuboid::new(
        Point3::new(-1.0, 0.5, -1.0),
        Vector3::new(1.0, 1.0, 1.0),
        Color::from_rgba(255, 102, 0, 255)));
    cuboid5.borrow_mut().look_at(
        Vector3::new(-2.0, 0.0, -2.0),
        Vector3::new(0.0, 1.0, 0.0));

    let cuboid6 = wrap!(Cuboid::new(
        Point3::new(-2.5, 0.25, -0.5),
        Vector3::new(0.5, 0.5, 0.5),
        Color::from_rgba(255, 204, 0, 255)));

    let platform = wrap!(Cuboid::new(
        Point3::new(0.0, -0.05, 0.0),
        Vector3::new(7.0, 0.1, 4.0),
        Color::from_rgba(153, 51, 255, 255)));

    let mut scene = Scene::new(camera);
    scene.add(Scene::node(&platform));
    scene.add(Scene::node(&entity));
    scene.add(Scene::node(&cuboid1));
    scene.add(Scene::node(&cuboid2));
    scene.add(Scene::node(&cuboid3));
    scene.add(Scene::node(&cuboid4));
    scene.add(Scene::node(&cuboid5));
    scene.add(Scene::node(&cuboid6));

    let animation_speed = 2.0;
    let camera_speed = 0.5;
    let cuboid3_scale = cuboid3.borrow().scale;
    let cuboid4_pos_x = cuboid4.borrow().position.x;

    cuboid6.borrow_mut().add(AntiClockwiseRotation::new(animation_speed));

    let mut ov = Overlay::new(800, 600);
    let wnd3 = Window::new("inner", WindowParams {
        pos: Vector2{x: Vector3::new(0.0, 0.0, 10.0), y: Vector3::new(0.0, 0.1, 0.0)},
        size: Vector2{x: Vector3::new(1.0, 0.0, -20.0), y: Vector3::new(0.0, 0.0, 40.0)},
        color: [Vector4::new(1.0, 1.0, 1.0, 1.0); 4],
        texcoord: [Vector2::zero(); 4],
        shown: true,
    });
    let wnd1 = Window::new("wnd1", WindowParams {
        pos: Vector2{x: Vector3::zero(), y: Vector3::zero()},
        size: Vector2{x: Vector3::new(0.2, 0.0, 0.0), y: Vector3::new(0.0, 1.0, 0.0)},
        color: [Vector4::new(0.8, 0.8, 0.5, 0.6); 4],
        texcoord: [Vector2::zero(); 4],
        shown: true,
    });
    let wnd2 = Window::new("wnd2", WindowParams {
        pos: Vector2{x: Vector3::new(0.2, 0.0, 10.0), y: Vector3::zero()},
        size: Vector2{x: Vector3::new(0.2, 0.0, -10.0), y: Vector3::new(0.0, 1.0, 0.0)},
        color: [Vector4::new(1.0, 0.5, 0.5, 0.9); 4],
        texcoord: [Vector2::zero(); 4],
        shown: true,
    });

    ov.root().attach(&wnd1);
    ov.root().attach(&wnd2);
    wnd1.attach(&wnd3);

    wnd1.child("inner").unwrap().detach();
    wnd2.attach(&wnd3);

    let window_ptr = &mut window as *mut glfw::Window;
    let input_mgr = Manager::new();

    let mut close_listener = KeyListener::new(key_mask![Key::Escape], false, move |_, _, action| {
            if action == Action::Press {
                unsafe { (*window_ptr).set_should_close(true); }
            }
        }
    );

    close_listener.gain_focus(&input_mgr);

    while !window.should_close() {
        Gliw::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        let mut time = glfw.get_time();

        cuboid3.borrow_mut().scale = cuboid3_scale +
            (f64::sin(time * animation_speed) as f32) * 0.75;

        cuboid4.borrow_mut().position.x = cuboid4_pos_x +
            f64::sin(time * animation_speed) as f32;

        // Clockwise rotation
        cuboid4_child_comp.borrow_mut().look_at(
            Vector3::new(
                f64::sin(time * animation_speed) as f32,
                0.0,
                f64::cos(time * animation_speed) as f32),
            Vector3::new(0.0, 1.0, 0.0));

        // Trigger the AntiClockwiseRotation component
        cuboid6.borrow_mut().emit(Event("rotate"), Data::from(&mut time));

        scene.camera_mut().look_at(
            Point3::<f32>::new(
                4.0 * f64::cos(time * camera_speed) as f32,
                3.0 * (f64::cos(time * camera_speed) * 0.5 + 1.0) as f32,
                6.0 * f64::sin(time * camera_speed) as f32),
            Point3::<f32>::new(0.0, 0.0, 0.0),
            Vector3::<f32>::new(0.0, 1.0, 0.0));

        wnd2.modify(|params| {
            params.size.x = Vector3::new(0.4 + 0.2*f32::sin(time as f32), 0.0, -10.0);
            params.color[0] = Vector4::new(0.75 - 0.25*f32::sin(time as f32), 0.2, 0.2, 0.9);
            params.color[1] = Vector4::new(1.0, 0.5 + 0.25*f32::sin(time as f32), 0.2, 0.9);
        });

        Gliw::enable(gl::DEPTH_TEST);
        scene.draw();

        Gliw::disable(gl::DEPTH_TEST);
        ov.draw();

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(key, scancode, action, _) => input_mgr.emit_key(key, scancode, action),
                _ => {}
            }
        }
    }
}


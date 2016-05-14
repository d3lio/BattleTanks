extern crate gl;
extern crate glfw;

use self::glfw::{Glfw, Window, Context};
use std::sync::{Mutex, Once, ONCE_INIT};

fn context() -> Window {
    let mut token = glfw().lock().unwrap();

    token.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    token.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    let (mut window, _) = token.create_window(800, 600, "", glfw::WindowMode::Windowed)
        .expect("Failed to create OpenGL window");

    window.make_current();
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    return window;
}

struct GlfwSingleton(*const Mutex<Glfw>, Once);
unsafe impl Sync for GlfwSingleton {}

fn glfw() -> &'static Mutex<Glfw> {
    unsafe {
        static mut internal: GlfwSingleton = GlfwSingleton(0 as *const _, ONCE_INIT);

        internal.1.call_once(|| {
            let token = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
            let mutex = Mutex::new(token);
            internal.0 = Box::into_raw(Box::new(mutex));
        });

        &*internal.0
    }
}

thread_local!(static CONTEXT: Window = context());

/// Initialize an OpenGL context and a Glfw window on the current thread.
/// Subsequent calls do nothing.
pub fn init_gl() {
    CONTEXT.with(|_| ());
}

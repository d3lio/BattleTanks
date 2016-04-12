extern crate gl;

#[repr(u32)]
pub enum DepthFunction {
    Never       = gl::NEVER,
    Less        = gl::LESS,
    Equal       = gl::EQUAL,
    LEqual      = gl::LEQUAL,
    Greater     = gl::GREATER,
    NotEqual    = gl::NOTEQUAL,
    GEqual      = gl::GEQUAL,
    Always      = gl::ALWAYS,
}

/// Wrapper for OpenGL misc functions.
pub struct Gliw;

impl Gliw {
    pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
        unsafe { gl::ClearColor(r, g, b, a); }
    }

    pub fn depth_func(df: DepthFunction) {
        unsafe { gl::DepthFunc(df as u32); }
    }

    pub fn enable(capability: u32) {
        unsafe { gl::Enable(capability); }
    }

    pub fn disable(capability: u32) {
        unsafe { gl::Disable(capability); }
    }

    pub fn clear(mask: u32) {
        unsafe { gl::Clear(mask); }
    }
}

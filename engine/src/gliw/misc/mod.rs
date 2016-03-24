extern crate gl;

pub struct Gliw;

impl Gliw {
    pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
        unsafe { gl::ClearColor(r, g, b, a); }
    }

    pub fn clear(mask: u32) {
        unsafe { gl::Clear(mask); }
    }
}

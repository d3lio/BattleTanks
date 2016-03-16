extern crate gl;

/// Wrapper for OpenGL Vertex Array Object
///
/// # References
/// * [Vertex Array Object](https://www.opengl.org/wiki/Vertex_Specification#Vertex_Array_Object)
pub struct Vao {
    handle: u32
}

impl Vao {
    /// Generates a vertex array
    pub fn new() -> Vao {
        let mut vao = Vao {
            handle: 0
        };

        unsafe { gl::GenVertexArrays(1, &mut vao.handle as *mut u32) }

        return vao;
    }

    /// Wrapper for `glBindVertexArray`
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.handle); }
    }

    /// Get the underlying OpenGL handle
    pub fn handle(&self) -> u32 {
        return self.handle;
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.handle); }
    }
}

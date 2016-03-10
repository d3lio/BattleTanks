extern crate gl;

use super::vbo::Vbo;

/// Wrapper for OpenGL VAO
///
/// #Example
/// ```no_run
/// let vao = Vao::new();
/// vao.add_vbo(
///     Vbo::new_from_data(
///         &VERTEX_DATA,
///         BufferType::Array,
///         BufferUsagePattern::StaticDraw));
/// ```
pub struct Vao {
    handle: u32,
    vbos: Vec<Vbo>
}

impl Vao {
    /// Generates a vertex array
    ///
    /// Does NOT bind self
    pub fn new() -> Vao {
        let mut vao = Vao {
            handle: 0,
            vbos: Vec::<Vbo>::new()
        };

        unsafe { gl::GenVertexArrays(1, &mut vao.handle as *mut u32) }

        return vao;
    }

    /// Adds a VBO to the VAO's vector
    pub fn add_vbo(&mut self, vbo: Vbo) -> &Self {
        self.bind();
        self.vbos.push(vbo);
        return self;
    }

    /// Get a VBO by index in the order it was added
    pub fn get_vbo(&self, idx: usize) -> &Vbo {
        return &self.vbos[idx];
    }

    /// Bind the VAO only
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.handle); }
    }

    /// Bind the VAO and all of it's VBOs
    pub fn bind_all(&self) {
        self.bind();

        for vbo in &self.vbos {
            vbo.bind();
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.handle); }
    }
}

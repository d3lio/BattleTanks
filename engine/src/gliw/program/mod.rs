extern crate gl;

pub mod builder;

/// Wrapper for a linked OpenGL Program.
///
/// Created using `ProgramBuilder` or `ProgramFromFileBuilder`.
pub struct Program {
    handle: u32,
}

impl Program {
    /// Wrapper for `glUseProgram`.
    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.handle); }
    }

    /// Get the underlying OpenGL handle.
    pub fn handle(&self) -> u32 {
        return self.handle;
    }
}

impl Drop for Program {
    fn drop (&mut self) {
        unsafe { gl::DeleteProgram(self.handle); }
    }
}

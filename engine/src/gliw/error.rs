extern crate gl;

/// Wrapper for error codes and their respective messages
pub struct Error {
    pub num: u32,
    pub msg: &'static str
}

pub const GL_OUT_OF_MEMORY: Error = Error { num: gl::OUT_OF_MEMORY, msg: "Unable to allocate memory" };
pub const GL_MAX_VERTEX_ATTRIBS: Error = Error { num: gl::MAX_VERTEX_ATTRIBS, msg: "Maximum number of vertex attributes exceeded"};

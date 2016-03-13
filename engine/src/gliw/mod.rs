//! GL Improvised Wrapper

mod program;
mod shader;
mod attrib_loc;

pub use self::shader::{Shader, ShaderType};
pub use self::program::{Program, ProgramBuilder, ProgramFromFileBuilder};
pub use self::attrib_loc::{AttribLocation};

//! GL Improvised Wrapper

mod program;
mod shader;

pub use self::shader::{Shader, ShaderType};
pub use self::program::{Program, ProgramBuilder, ProgramFromFileBuilder};

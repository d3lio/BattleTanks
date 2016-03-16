//! GL Improvised Wrapper
//!
//! # Remarks
//! * Does not support immutable storage yet for both buffers and textures.

mod error;
mod program;
mod shader;
mod texture;
mod vao;
mod vbo;

pub use self::program::{Program, ProgramBuilder, ProgramFromFileBuilder};
pub use self::shader::{Shader, ShaderType};
pub use self::texture::{Texture, TextureType};
pub use self::vao::Vao;
pub use self::vbo::{Vbo, BufferType, BufferUsagePattern};

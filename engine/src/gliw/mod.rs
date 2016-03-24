//! GL Improvised Wrapper
//!
//! # Remarks
//! * Does not support immutable storage for any OpenGL objects yet.

mod misc;
mod program;
mod shader;
mod texture;
mod uniform;
mod vao;
mod vbo;
mod vert_attrib;

mod error;

pub use self::misc::Gliw;
pub use self::program::{Program, Uniform};
pub use self::program::builder::{ProgramBuilder, ProgramFromFileBuilder};
pub use self::shader::{Shader, ShaderType};
pub use self::texture::{Texture, TextureType, TextureBuilder2D, ImageType, TextureCoordWrap, TextureFilter};
pub use self::uniform::{UniformData};
pub use self::vao::Vao;
pub use self::vbo::{Vbo, BufferType, BufferUsagePattern};
pub use self::vert_attrib::{VertexAttrib, AttribFloatFormat, AttribIntFormat};

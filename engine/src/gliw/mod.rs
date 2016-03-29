//! GL Improvised Wrapper
//!
//! # Remarks
//! * Does not support immutable storage for any OpenGL objects yet.

mod buffer;
mod misc;
mod program;
mod shader;
mod texture;
mod uniform;
mod vao;
mod vert_attrib;

mod error;

pub use self::buffer::{Buffer, BufferType, BufferUsagePattern};
pub use self::misc::{Gliw, DepthFunction};
pub use self::program::{Program, Uniform};
pub use self::program::builder::{ProgramBuilder, ProgramFromFileBuilder};
pub use self::shader::{Shader, ShaderType};
pub use self::texture::{Texture, TextureType};
pub use self::texture::builder::{TextureBuilder2D, ImageType, TextureCoordWrap, TextureFilter};
pub use self::uniform::{UniformData};
pub use self::vao::Vao;
pub use self::vert_attrib::{VertexAttrib, AttribFloatFormat, AttribIntFormat};

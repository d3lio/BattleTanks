//! Texture module

//mod texture;
mod tex_builder;

//pub use self::texture::{Texture, TextureType};
pub use self::tex_builder::{TextureBuilder2D, ImageType, TextureCoordWrap, TextureFilter};

extern crate gl;

use gliw::error;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureType {
    Tex1D               = gl::TEXTURE_1D,
    Tex2D               = gl::TEXTURE_2D,
    Tex3D               = gl::TEXTURE_3D,
    Array1D             = gl::TEXTURE_1D_ARRAY,
    Array2D             = gl::TEXTURE_2D_ARRAY,
    Rectangle           = gl::TEXTURE_RECTANGLE,
    CubeMap             = gl::TEXTURE_CUBE_MAP,
    CubeMapArray        = gl::TEXTURE_CUBE_MAP_ARRAY,
    Buffer              = gl::TEXTURE_BUFFER,
    Multisample2D       = gl::TEXTURE_2D_MULTISAMPLE,
    MultisampleArray2D  = gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
}

/// Wrapper for OpenGL Texture Object
///
/// # References
/// * [Texture Object](https://www.opengl.org/wiki/Texture)
/// * [Array texture](https://www.opengl.org/wiki/Array_Texture)
/// * [Texture storage](https://www.opengl.org/wiki/Texture_Storage)
/// * [Image format](https://www.opengl.org/wiki/Image_Format)
pub struct Texture {
    handle: u32,
    tex_type: TextureType
}

impl Texture {
    /// Generates a texture and set it's type (target) for safe future gl function calls
    pub fn new(tex_type: TextureType) -> Texture {
        let mut tex = Texture {
            handle: 0,
            tex_type: tex_type
        };

        unsafe { gl::GenTextures(1, &mut tex.handle as *mut u32); }

        return tex;
    }

    /// Wrapper for `glBindTexture`
    pub fn bind(&self) {
        unsafe { gl::BindTexture(self.tex_type as u32, self.handle); }
    }

    /// Passes the texture the the given program and location on tex_unit
    // pub fn pass_to(&self, prog: Program, location: Uniform, tex_unit: u32) {
    //     unsafe {
    //         if tex_unit >= gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS {
    //             panic!(error::GL_INVALID_ENUM.msg);
    //         }
    //         gl::ActiveTexture(gl::TEXTURE0 + tex_unit);
    //     }
    //     self.bind();
    //     location...
    // }

    /// Get the texture's type (target)
    pub fn tex_type(&self) -> TextureType {
        return self.tex_type;
    }

    /// Get the underlying OpenGL handle
    pub fn handle(&self) -> u32 {
        return self.handle;
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.handle); }
    }
}


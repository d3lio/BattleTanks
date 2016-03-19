extern crate gl;

use super::{Texture, TextureType};

use std::fs::File;
use std::io::{Read, ErrorKind};
use std::os::raw::c_void;

pub enum ImageType {
    Bmp,
}

#[repr(u32)]
pub enum TextureCoordWrap {
    Repeat          = gl::REPEAT,
    MirroredRepeat  = gl::MIRRORED_REPEAT,
    ClampToEdge     = gl::CLAMP_TO_EDGE,
    ClampToBorder   = gl::CLAMP_TO_BORDER,
}

/// Texture coordinates filtering methods
///
/// When using mipmaps filter *Mipmap* the first asterisk
/// stands for the mipmap resolving method. The second asterisk
/// stands for the coordinates filtering method itself.
#[repr(u32)]
pub enum TextureFilter {
    Nearest               = gl::NEAREST,
    Linear                = gl::LINEAR,
    NearestMipmapNearest  = gl::NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest   = gl::LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear   = gl::NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear    = gl::LINEAR_MIPMAP_LINEAR,
}

/// A builder class for loading 2D textures from files
///
/// Please use the builder's methods in the order shown below because
/// in some cases the ordering matters and this is the most optimal order
/// insuring that the outcome is the expected one.
///
/// # Examples
///
/// Load a bitmap:
///
/// ```ignore
/// let program = // ...obtain program somehow
/// let uniform_location = // TODO
/// let tex = TextureBuilder2D::new()
///     .source("path/to/image", ImageType::Bmp)
///     .wrap(TextureCoordWrap::Repeat, TextureCoordWrap::Repeat)
///     .filter(TextureFilter::LinearMipmapLinear, TextureFilter::Linear)
///     .load();
/// tex.pass_to(program, uniform_location, 0);
/// ```
pub struct TextureBuilder2D {
    texture: Texture,
    /// Not using `&str` to avoid lifetimes and achieve clean code
    path: String,
    img_type: ImageType
}

impl TextureBuilder2D {
    pub fn new() -> TextureBuilder2D {
        return TextureBuilder2D {
            texture: Texture::new(TextureType::Tex2D),
            path: String::from(""),
            img_type: ImageType::Bmp
        }
    }

    /// Specifies the path to the image and it's type
    pub fn source(&mut self, path: &str, img_type: ImageType) -> &mut Self {
        self.path = String::from(path);
        self.img_type = img_type;
        return self;
    }

    /// Specifies the wrapping method for S and T texture coordinates
    ///
    /// Usually `Repeat` is preffered
    pub fn wrap(&mut self, s_wm: TextureCoordWrap, t_wm: TextureCoordWrap) -> &mut Self {
        self.texture.bind();
        unsafe {
            gl::TexParameteri(self.texture.tex_type() as u32, gl::TEXTURE_WRAP_S, s_wm as i32);
            gl::TexParameteri(self.texture.tex_type() as u32, gl::TEXTURE_WRAP_T, t_wm as i32);
        }
        return self;
    }

    /// Specifies the filtering method to use when scaling the image
    ///
    /// Use `Nearest` to achieve pixelized effect
    /// Use `Linear` to achieve smoothness
    pub fn filter(&mut self, min_filter: TextureFilter, mag_filter: TextureFilter) -> &mut Self {
        self.texture.bind();
        unsafe {
            gl::TexParameteri(self.texture.tex_type() as u32, gl::TEXTURE_MIN_FILTER, min_filter as i32);
            gl::TexParameteri(self.texture.tex_type() as u32, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
        }
        return self;
    }

    /// Wrapper for `glGenerateMipmap`
    pub fn gen_mipmap(&mut self) -> &mut Self {
        self.texture.bind();
        unsafe { gl::GenerateMipmap(self.texture.tex_type() as u32); }
        return self;
    }

    /// Middleware for executing arbitrary code
    ///
    /// Useful for situational code like rarely used parameters
    pub fn middleware<F>(&mut self, closure: F) -> &mut Self where F: Fn(&Texture) {
        self.texture.bind();
        closure(&self.texture);
        return self;
    }

    /// Loads the data from the file and passes it to OpenGL
    pub fn load(&self) -> Result<Texture, String> {
        self.texture.bind();

        let load_res = match self.img_type {
            ImageType::Bmp => self.load_bmp()
        };

        return match load_res {
            Some(err) => Err(err),
            None => Ok(Texture {
                handle: self.texture.handle(),
                tex_type: self.texture.tex_type()
            })
        }
    }

    fn load_bmp(&self) -> Option<String> {
        const BMP_HEADER_SIZE: usize = 54;
        let mut header: [u8; BMP_HEADER_SIZE] = [0; BMP_HEADER_SIZE];

        let mut file;

        // Open the file
        match File::open(&self.path) {
            Err(err) => {
                if err.kind() == ErrorKind::UnexpectedEof {
                    return Some(String::from(INCORRECT_FORMAT));
                }
                return Some(String::from(format!("{}", err)));
            }
            Ok(f) => file = f
        }

        // Read the header
        match file.read_exact(&mut header) {
            Err(err) => return Some(String::from(format!("{}", err))),
            Ok(_) => ()
        }

        // Check if the format is truely bitmap
        if header[0] != b'B' || header[1] != b'M' {
            return Some(String::from(INCORRECT_FORMAT));
        }

        // Macro for easy header properties extraction
        macro_rules! fprop {
            ($prop: expr) => {
                unsafe { *(& $prop as *const u8 as *const i32) }
            }
        }

        // Check if the image is 24bpp
        if fprop!(header[0x1E]) != 0 ||
           fprop!(header[0x1C]) != 24 {
            return Some(String::from(INCORRECT_FORMAT));
        }

        // Extract the information about the image
        // let mut data_pos    = fprop!(header[0x0A]);
        let mut image_size  = fprop!(header[0x22]);
        let width           = fprop!(header[0x12]);
        let height          = fprop!(header[0x16]);

        // Some BMP files are misformatted
        // if data_pos == 0   { data_pos = BMP_HEADER_SIZE as i32; }
        if image_size == 0 { image_size = width * height * 3; }

        let mut data: Vec<u8> = Vec::<u8>::new();

        // Read the data
        match file.read_to_end(&mut data) {
            Err(err) => return Some(String::from(format!("{}", err))),
            Ok(size) => if size != image_size as usize {
                return Some(String::from(INCORRECT_FORMAT));
            }
        }

        // Load the texture
        unsafe {
            gl::TexImage2D(
                self.texture.tex_type() as u32,
                0,
                gl::RGB as i32,
                width,
                height,
                0,
                gl::BGR,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void
            );
        }

        return None;
    }
}

const INCORRECT_FORMAT: &'static str = "Incorrect file format";

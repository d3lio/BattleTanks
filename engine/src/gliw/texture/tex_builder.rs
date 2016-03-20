extern crate gl;

use super::{Texture, TextureType};

use std::fs::File;
use std::io::{Read, ErrorKind};
use std::os::raw::c_void;

pub enum ImageType {
    Bmp,
}

#[repr(u32)]
#[derive(Copy, Clone)]
/// Wrapping methods for texture coordinates.
pub enum TextureCoordWrap {
    Repeat             = gl::REPEAT,
    MirroredRepeat     = gl::MIRRORED_REPEAT,
    ClampToEdge        = gl::CLAMP_TO_EDGE,
    ClampToBorder      = gl::CLAMP_TO_BORDER,
    MirrorClampToEdge  = gl::MIRROR_CLAMP_TO_EDGE,
}

/// Filtering methods for texture coordinates.
///
/// When using mipmaps filter {\*}Mipmap{\*}:
///
/// * The first asterisk stands for the mipmap resolving method.
/// * The second asterisk stands for the coordinates filtering method itself.
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum TextureFilter {
    None                  = 0,
    Nearest               = gl::NEAREST,
    Linear                = gl::LINEAR,
    NearestMipmapNearest  = gl::NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest   = gl::LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear   = gl::NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear    = gl::LINEAR_MIPMAP_LINEAR,
}

/// A builder class for loading 2D textures from files.
///
/// # Important
///
/// Be sure to load power-of-two dimensions texture like 16x16, 128x128, 64x256, etc.
/// The code does not restrict you to do so but will produce some broken textures otherwise.
///
/// # Examples
///
/// Load a bitmap (.bmp):
///
/// ```no_run
/// # use engine::gliw::{
/// #   Program, ProgramBuilder,
/// #   TextureBuilder2D, ImageType, TextureCoordWrap, TextureFilter
/// # };
/// let program: Program; // ...obtain a program somehow
/// # program = ProgramBuilder::new().link().unwrap();
/// let tex = TextureBuilder2D::new()
///     .source("pink_panther.bmp", ImageType::Bmp)
///     .wrap(TextureCoordWrap::Repeat, TextureCoordWrap::Repeat)
///     .filter(TextureFilter::LinearMipmapLinear, TextureFilter::Linear)
///     .gen_mipmap()
///     .load()
///     .unwrap();
///
///     tex.pass_to(&program, "tex", 0);
/// ```
///
/// Middleware example:
///
/// ```no_run
/// # use engine::gliw::{
/// #   Program, ProgramBuilder,
/// #   TextureBuilder2D, ImageType, TextureCoordWrap, TextureFilter
/// # };
/// let program: Program; // ...obtain a program somehow
/// # program = ProgramBuilder::new().link().unwrap();
/// let tex = TextureBuilder2D::new()
///     .source("pink_panther.bmp", ImageType::Bmp)
///     .wrap(TextureCoordWrap::Repeat, TextureCoordWrap::Repeat)
///     .filter(TextureFilter::LinearMipmapLinear, TextureFilter::Linear)
///     .gen_mipmap()
///     .middleware(|tex| { /* Some arbitrary code here... */ })
///     .middleware(|tex| unsafe {
///         /* Some unsafe arbitrary code here... */
///     })
///     .load()
///     .unwrap();
///
///     tex.pass_to(&program, "tex", 0);
/// ```
pub struct TextureBuilder2D {
    s_wrap: TextureCoordWrap,
    t_wrap: TextureCoordWrap,
    min_filter: TextureFilter,
    mag_filter: TextureFilter,
    gen_mipmap: bool,
    middleware: Vec<Box<Fn(&Texture)>>,
    path: String,
    img_type: ImageType
}

impl TextureBuilder2D {
    pub fn new() -> TextureBuilder2D {
        return TextureBuilder2D {
            s_wrap: TextureCoordWrap::Repeat,
            t_wrap: TextureCoordWrap::Repeat,
            min_filter: TextureFilter::None,
            mag_filter: TextureFilter::None,
            gen_mipmap: false,
            middleware: Vec::<Box<Fn(&Texture)>>::new(),
            path: String::from(""),
            img_type: ImageType::Bmp
        }
    }

    /// Specifies the path to the image and it's type.
    pub fn source(&mut self, path: &str, img_type: ImageType) -> &mut Self {
        self.path = String::from(path);
        self.img_type = img_type;
        return self;
    }

    /// Specifies the wrapping method for S and T texture coordinates.
    ///
    /// Initially the wrap methods are set to `Repeat`
    /// in both OpenGL and this implementation.
    pub fn wrap(&mut self, s_wrap: TextureCoordWrap, t_wrap: TextureCoordWrap) -> &mut Self {
        self.s_wrap = s_wrap;
        self.t_wrap = t_wrap;
        return self;
    }

    /// Specifies the filtering method to use when scaling the image.
    ///
    /// Use `Nearest` to achieve pixelized effect.<br>
    /// Use `Linear` to achieve smoothness.
    ///
    /// **Note:** `mag_filter` can only be `Nearest` or `Linear`.
    /// Otherwise it will be set to `TextureFilter::None`.
    pub fn filter(&mut self, min_filter: TextureFilter, mag_filter: TextureFilter) -> &mut Self {
        self.min_filter = min_filter;
        self.mag_filter = match mag_filter {
            TextureFilter::Linear | TextureFilter::Nearest => mag_filter,
            _ => TextureFilter::None
        };
        return self;
    }

    /// Wrapper for `glGenerateMipmap`.
    pub fn gen_mipmap(&mut self) -> &mut Self {
        self.gen_mipmap = true;
        return self;
    }

    /// Middleware for executing arbitrary code.
    ///
    /// Useful for situational code like anisotropy filtering.
    /// **Note:** middleware will always be called after the texture has been loaded to OpenGL
    /// and after all other standard builder methods have been called.
    /// Also it's guaranteed to have the texture bound before execution of each middleware.
    pub fn middleware<F: 'static>(&mut self, closure: F) -> &mut Self where F: Fn(&Texture) {
        self.middleware.push(Box::new(closure));
        return self;
    }

    /// Loads the data from the file and passes it to OpenGL.
    pub fn load(&mut self) -> Result<Texture, String> {
        let tex = Texture::new(TextureType::Tex2D);

        tex.bind();

        // Resolve loading method
        let load_res = match self.img_type {
            ImageType::Bmp => self.load_bmp(&tex)
        };

        if let Some(err) = load_res {
            return Err(err);
        }

        unsafe {
            tex.bind();

            gl::TexParameteri(tex.tex_type() as u32, gl::TEXTURE_WRAP_S, self.s_wrap as i32);
            gl::TexParameteri(tex.tex_type() as u32, gl::TEXTURE_WRAP_T, self.t_wrap as i32);

            match self.min_filter {
                TextureFilter::None => (),
                _ => gl::TexParameteri(tex.tex_type() as u32, gl::TEXTURE_MIN_FILTER, self.min_filter as i32)
            }

            match self.mag_filter {
                TextureFilter::None => (),
                _ => gl::TexParameteri(tex.tex_type() as u32, gl::TEXTURE_MAG_FILTER, self.mag_filter as i32)
            }

            if self.gen_mipmap {
                gl::GenerateMipmap(tex.tex_type() as u32);
            }
        }

        // Execute all of the collected closures
        for closure_box in &self.middleware {
            tex.bind();
            (*closure_box)(&tex);
        }

        return Ok(tex);
    }

    /// As of now it only loads 24bpp bitmaps.
    fn load_bmp(&self, tex: &Texture) -> Option<String> {
        const BMP_HEADER_SIZE: usize = 54;
        let mut header: [u8; BMP_HEADER_SIZE] = [0; BMP_HEADER_SIZE];

        let mut file;

        // Open the file
        match File::open(&self.path) {
            Err(err) => return Some(String::from(format!("{}", err))),
            Ok(f) => file = f
        }

        // Read the header
        match file.read_exact(&mut header) {
            Err(ref err) if err.kind() == ErrorKind::UnexpectedEof =>
                return Some(String::from(INCORRECT_FORMAT)),
            Err(err) =>
                return Some(String::from(format!("{}", err))),
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
                tex.tex_type() as u32,
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

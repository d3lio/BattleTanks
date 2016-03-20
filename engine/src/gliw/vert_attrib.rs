extern crate gl;

use gliw::{Vao, Vbo, Program};
use gliw::error;

use std::ffi::CString;
use std::os::raw::c_void;

/// Data formats for `VertexAttrib::data_float_format`
///
/// All formats are represented by two values:
///
///  * `size` - the number of components: 1, 2, 3 or 4
///  * `normalized` - specifies whether values should be normalized (to the range [-1f; 1f] for signed
///    or [0f; 1f] for unsigned) or converted directly
///
/// For variants where there is only one possible value for `size` or `normalized` those field are omitted.
/// This avoid some of the possibilities to pass invalid values to `data_float_format`.
///
/// OpenGL accepts the symbolic constant `GL_BGRA` for size. To use that use one of the *_BGRA enum variants.
///
#[allow(non_camel_case_types)]
pub enum AttribFloatFormat {
    /// tuple `Byte(size, normalized)`
    Byte(i32, bool),
    /// tuple `Ubyte(size, normalized)`
    Ubyte(i32, bool),
    /// tuple `Short(size, normalized)`
    Short(i32, bool),
    /// tuple `Ushort(size, normalized)`
    Ushort(i32, bool),
    /// tuple `Int(size, normalized)`
    Int(i32, bool),
    /// tuple `Uint(size, normalized)`
    Uint(i32, bool),

    /// tuple `HalfFloat(size)`, normalized = false
    HalfFloat(i32),
    /// tuple `Float(size)`, normalized = false
    Float(i32),
    /// tuple `Double(size)`, normalized = false
    Double(i32),
    /// tuple `Fixed(size)`, normalized = false
    Fixed(i32),

    /// tuple `Int_2_10_10_10_Rev(normalized)`, size = 4
    Int_2_10_10_10_Rev(bool),
    /// tuple `Uint_2_10_10_10_Rev(normalized)`, size = 4
    Uint_2_10_10_10_Rev(bool),
    /// tuple `Uint_10f_11f_11f_Rev(normalized)`, size = 3
    Uint_10f_11f_11f_Rev(bool),

    /// size = GL_BGRA, normalized = true
    Ubyte_BGRA,
    /// size = GL_BGRA, normalized = true
    Int_2_10_10_10_Rev_BGRA,
    /// size = GL_BGRA, normalized = true
    Uint_2_10_10_10_Rev_BGRA,
}

/// Data formats for `VertexAttrib::data_float_format`
///
/// All formats are represented by a tuple with a single field `size` - the number of components: 1, 2, 3 or 4
pub enum AttribIntFormat {
    Byte(i32),
    Ubyte(i32),
    Short(i32),
    Ushort(i32),
    Int(i32),
    Uint(i32),
}

/// Wrapper for OpenGL Attribute Location.
///
/// Note that this class does not give static guarantees that an actual attribute exists.
/// This is because the return value of `glGetAttribLocation` is ambiguous - a value of `-1`
/// can mean that either no variable with the given name exists, or that it exists but is unused,
/// so it has been optimized out by the driver.
pub struct VertexAttrib {
    handle: i32,
}

impl VertexAttrib {
    /// Create a `VertexAttrib` directly from a given handle.
    ///
    /// Use only if you have specified the location yourself, for example by
    /// using GLSL layout qualifiers `layout(location=<index>)`
    pub fn new(handle: i32) -> VertexAttrib {
        return VertexAttrib {
            handle: handle,
        };
    }

    /// Wrapper for `glVertexAttribPointer`
    ///
    /// Specifies the format in which data from `vbo` will be read for the vertex attribute.
    /// Use this function for floating point vertex attributes - `float`, `double`, `vec*`, `dvec*`, `mat*`
    ///
    /// # Examples
    ///
    ///
    /// ```no_run
    /// # use engine::gliw::{VertexAttrib, AttribFloatFormat, Vao, Vbo, BufferType};
    /// # use std::ptr;
    /// # let vao = Vao::new();
    /// # let vbo = Vbo::new(BufferType::Array);
    /// # let attrib = VertexAttrib::new(-1);
    /// // Populate a shader variable of type `vec3` from a vbo containing `[f32; 3]`
    /// attrib.data_float_format(&vao, &vbo, AttribFloatFormat::Float(3), 0, ptr::null());
    ///
    /// // Populate a shader variable of type `vec3` from a vbo containing `[u8; 3]`, mapping values in the range [0, 255] to [0f, 1f]
    /// attrib.data_float_format(&vao, &vbo, AttribFloatFormat::Ubyte(3, true), 0, ptr::null());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `stride < 0`. <br>
    /// Panics if `size` of `format` is not between 1 and 4. <br>
    /// Panics if the attribute hande is greater than or equal to `GL_MAX_VERTEX_ATTRIBS`. <br>
    pub fn data_float_format(&self, vao: &Vao, vbo: &Vbo, format: AttribFloatFormat, stride: i32, offset: *const c_void) {
        if stride < 0 {
            panic!(NEGATIVE_STRIDE);
        }

        unsafe {
            let mut max_vertex_attribs: i32 = 0;
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attribs);
            if self.handle >= max_vertex_attribs {
                panic!(error::GL_MAX_VERTEX_ATTRIBS.msg);
            }
        }

        vao.bind();
        vbo.bind();

        match format {
            AttribFloatFormat::Byte(size @ 1...4, normalized)    => unsafe { gl::VertexAttribPointer(self.handle as u32, size, gl::BYTE, normalized as u8, stride, offset); },
            AttribFloatFormat::Ubyte(size @ 1...4, normalized)   => unsafe { gl::VertexAttribPointer(self.handle as u32, size, gl::UNSIGNED_BYTE, normalized as u8, stride, offset); },
            AttribFloatFormat::Short(size @ 1...4, normalized)   => unsafe { gl::VertexAttribPointer(self.handle as u32, size, gl::SHORT, normalized as u8, stride, offset); },
            AttribFloatFormat::Ushort(size @ 1...4, normalized)  => unsafe { gl::VertexAttribPointer(self.handle as u32, size, gl::UNSIGNED_SHORT, normalized as u8, stride, offset); },
            AttribFloatFormat::Int(size @ 1...4, normalized)     => unsafe { gl::VertexAttribPointer(self.handle as u32, size, gl::INT, normalized as u8, stride, offset); },
            AttribFloatFormat::Uint(size @ 1...4, normalized)    => unsafe { gl::VertexAttribPointer(self.handle as u32, size, gl::UNSIGNED_INT, normalized as u8, stride, offset); },

            AttribFloatFormat::HalfFloat(size @ 1...4)           => unsafe { gl::VertexAttribPointer(self.handle as u32, size, gl::HALF_FLOAT, gl::FALSE, stride, offset); },
            AttribFloatFormat::Float(size @ 1...4)               => unsafe { gl::VertexAttribPointer(self.handle as u32, size, gl::FLOAT, gl::FALSE, stride, offset); },
            AttribFloatFormat::Double(size @ 1...4)              => unsafe { gl::VertexAttribPointer(self.handle as u32, size, gl::DOUBLE, gl::FALSE, stride, offset); },
            AttribFloatFormat::Fixed(size @ 1...4)               => unsafe { gl::VertexAttribPointer(self.handle as u32, size, gl::FIXED, gl::FALSE, stride, offset); },

            AttribFloatFormat::Int_2_10_10_10_Rev(normalized)    => unsafe { gl::VertexAttribPointer(self.handle as u32, 4, gl::INT_2_10_10_10_REV, normalized as u8, stride, offset); },
            AttribFloatFormat::Uint_2_10_10_10_Rev(normalized)   => unsafe { gl::VertexAttribPointer(self.handle as u32, 4, gl::UNSIGNED_INT_2_10_10_10_REV, normalized as u8, stride, offset); },
            AttribFloatFormat::Uint_10f_11f_11f_Rev(normalized)  => unsafe { gl::VertexAttribPointer(self.handle as u32, 3, gl::UNSIGNED_INT_10F_11F_11F_REV, normalized as u8, stride, offset); },

            AttribFloatFormat::Ubyte_BGRA                        => unsafe { gl::VertexAttribPointer(self.handle as u32, gl::BGRA as i32, gl::UNSIGNED_BYTE, gl::TRUE, stride, offset); },
            AttribFloatFormat::Int_2_10_10_10_Rev_BGRA           => unsafe { gl::VertexAttribPointer(self.handle as u32, gl::BGRA as i32, gl::INT_2_10_10_10_REV, gl::TRUE, stride, offset); },
            AttribFloatFormat::Uint_2_10_10_10_Rev_BGRA          => unsafe { gl::VertexAttribPointer(self.handle as u32, gl::BGRA as i32, gl::UNSIGNED_INT_2_10_10_10_REV, gl::TRUE, stride, offset); },

            _ => { panic!(INVALID_DATA_SIZE); },
        }
    }

    /// Wrapper for `glVertexAttribIPointer`
    ///
    /// Specifies the format in which data from `vbo` will be read for the vertex attribute. Use this function
    /// for integer types - `bool`, `int`, `uint`, `bvec*`, `ivec*`, `uvec*`
    ///
    /// # Panics
    /// Same as `data_float_format`
    pub fn data_int_format(&self, vao: &Vao, vbo: &Vbo, format: AttribIntFormat, stride: i32, offset: *const c_void) {
        vao.bind();
        vbo.bind();

        if stride < 0 {
            panic!(NEGATIVE_STRIDE);
        }

        unsafe {
            let mut max_vertex_attribs: i32 = 0;
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attribs);
            if self.handle >= max_vertex_attribs {
                panic!(error::GL_MAX_VERTEX_ATTRIBS.msg);
            }
        }


        match format {
            AttribIntFormat::Byte(size @ 1...4)    => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::BYTE, stride, offset); },
            AttribIntFormat::Ubyte(size @ 1...4)   => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::UNSIGNED_BYTE, stride, offset); },
            AttribIntFormat::Short(size @ 1...4)   => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::SHORT, stride, offset); },
            AttribIntFormat::Ushort(size @ 1...4)  => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::UNSIGNED_SHORT, stride, offset); },
            AttribIntFormat::Int(size @ 1...4)     => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::INT, stride, offset); },
            AttribIntFormat::Uint(size @ 1...4)    => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::UNSIGNED_INT, stride, offset); },

            _ => { panic!(INVALID_DATA_SIZE); },
        }
    }

    /// Wrapper for `glEnableVertexAttribArray`
    pub fn enable(&self, vao: &Vao) {
        vao.bind();
        unsafe { gl::EnableVertexAttribArray(self.handle as u32); }
    }

    /// Wrapper for `glDisableVertexAttribArray`
    pub fn disable(&self, vao: &Vao) {
        vao.bind();
        unsafe { gl::DisableVertexAttribArray(self.handle as u32); }
    }

    /// Get the underlying OpenGL handle
    pub fn handle(&self) -> i32 {
        return self.handle;
    }
}

impl Program {
    /// Wrapper for `glGetAttribLocation`
    pub fn get_attrib_loc(&self, name: &str) -> VertexAttrib {
        unsafe {
            let loc = gl::GetAttribLocation(self.handle(), CString::new(name).unwrap().as_ptr());
            return VertexAttrib {
                handle: loc,
            }
        }
    }
}

const NEGATIVE_STRIDE: &'static str = "Stride must be nonnegative";
const INVALID_DATA_SIZE: &'static str = "Invalid data format - size must be 1, 2, 3 or 4";

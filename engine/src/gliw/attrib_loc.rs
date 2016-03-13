extern crate gl;

use std::ffi::CString;
use std::os::raw::c_void;
use gliw::Program;

#[allow(non_camel_case_types)]
pub enum AttribFloatFormat {
    Byte(i32, bool),
    Ubyte(i32, bool),
    Short(i32, bool),
    Ushort(i32, bool),
    Int(i32, bool),
    Uint(i32, bool),

    HalfFloat(i32),
    Float(i32),
    Double(i32),
    Fixed(i32),

    Int_2_10_10_10_Rev(bool),   // size = 4
    Uint_2_10_10_10_Rev(bool),  // size = 4
    Uint_10f_11f_11f_Rev(bool), // size = 3

    Ubyte_BGRA,                 // size = GL_BGRA, normalized = false
    Int_2_10_10_10_Rev_BGRA,    // size = GL_BGRA, normalized = false
    Uint_2_10_10_10_Rev_BGRA,   // size = GL_BGRA, normalized = false
}

pub enum AttribIntFormat {
    Byte(i32),
    Ubyte(i32),
    Short(i32),
    Ushort(i32),
    Int(i32),
    Uint(i32),
}

/// Wrapper around an OpenGL attribute location.
pub struct AttribLocation {
    handle: i32,
}

impl AttribLocation {
    // pub fn new(handle) -> AttribLocation {
    // }

    pub fn handle(&self) -> i32 {
        return self.handle;
    }

    // TODO: doc
    /// Wrapper for `glVertexAttribPointer`
    pub fn data_float_format(&self, /*vao: &Vao, vbo: &Vbo,*/ format: AttribFloatFormat, stride: i32, offset: *const c_void) {
        // vao.bind();
        // vbo.bind();

        if stride < 0 {
            panic!("stride cannot be negative");
        }

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

            _ => { panic!("invalid data format - size must be between 1 and 4"); },
        }
    }

    // TODO: doc
    /// Wrapper for `glVertexAttribIPointer`
    pub fn data_int_format(&self, /*vao: &Vao, vbo: &Vbo,*/ format: AttribIntFormat, stride: i32, offset: *const c_void) {
        // vao.bind();
        // vbo.bind();

        if stride < 0 {
            panic!("stride cannot be negative");
        }

        match format {
            AttribIntFormat::Byte(size @ 1...4)    => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::BYTE, stride, offset); },
            AttribIntFormat::Ubyte(size @ 1...4)   => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::UNSIGNED_BYTE, stride, offset); },
            AttribIntFormat::Short(size @ 1...4)   => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::SHORT, stride, offset); },
            AttribIntFormat::Ushort(size @ 1...4)  => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::UNSIGNED_SHORT, stride, offset); },
            AttribIntFormat::Int(size @ 1...4)     => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::INT, stride, offset); },
            AttribIntFormat::Uint(size @ 1...4)    => unsafe { gl::VertexAttribIPointer(self.handle as u32, size, gl::UNSIGNED_INT, stride, offset); },

            _ => { panic!("invalid data format - size must be between 1 and 4"); },
        }
    }
}

impl Program {
    /// Wrapper for glGetAttribLocation()
    pub fn get_attrib_loc(&self, name: &str) -> AttribLocation {
        unsafe {
            let loc = gl::GetAttribLocation(self.handle(), CString::new(name).unwrap().as_ptr());

            // Note that `loc == -1` means that either no variable with name `name` exists,
            // or that it exists but is unused, so it has been optimised out by the driver.
            // Because of this we can't give static guarantees like most other wrapper objects
            return AttribLocation {
                handle: loc,
            }
        }
    }
}

extern crate gl;

use gliw::Program;

use std::ffi::CString;

pub struct Uniform {
    handle: i32,
}

pub enum UniformData<'a> {
    Float1(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Float4(f32, f32, f32, f32),

    Int1(i32),
    Int2(i32, i32),
    Int3(i32, i32, i32),
    Int4(i32, i32, i32, i32),

    Uint1(u32),
    Uint2(u32, u32),
    Uint3(u32, u32, u32),
    Uint4(u32, u32, u32, u32),

    FloatVec(i32, &'a [f32]),
    IntVec(i32, &'a [i32]),
    UintVec(i32, &'a [u32]),

    FloatMat(i32, bool, &'a [f32]),
    FloatMatNxM(i32, i32, bool, &'a [f32]),
}

impl Uniform {
    pub fn value<'a> (&'a self, data: UniformData<'a>) {
        match data {
            UniformData::Float1(x) => unsafe { gl::Uniform1f(self.handle, x); },
            UniformData::Float2(x, y) => unsafe { gl::Uniform2f(self.handle, x, y); },
            UniformData::Float3(x, y, z) => unsafe { gl::Uniform3f(self.handle, x, y, z); },
            UniformData::Float4(x, y, z, w) => unsafe { gl::Uniform4f(self.handle, x, y, z, w); },

            UniformData::Int1(x) => unsafe { gl::Uniform1i(self.handle, x); },
            UniformData::Int2(x, y) => unsafe { gl::Uniform2i(self.handle, x, y); },
            UniformData::Int3(x, y, z) => unsafe { gl::Uniform3i(self.handle, x, y, z); },
            UniformData::Int4(x, y, z, w) => unsafe { gl::Uniform4i(self.handle, x, y, z, w); },

            UniformData::Uint1(x) => unsafe { gl::Uniform1ui(self.handle, x); },
            UniformData::Uint2(x, y) => unsafe { gl::Uniform2ui(self.handle, x, y); },
            UniformData::Uint3(x, y, z) => unsafe { gl::Uniform3ui(self.handle, x, y, z); },
            UniformData::Uint4(x, y, z, w) => unsafe { gl::Uniform4ui(self.handle, x, y, z, w); },

            UniformData::FloatVec(1, arr) => {
                unsafe { gl::Uniform1fv(self.handle, arr.len() as i32, arr.as_ptr()); }
            },
            UniformData::FloatVec(2, arr) => {
                if arr.len() % 2 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::Uniform2fv(self.handle, arr.len() as i32 / 2, arr.as_ptr()); }
            },
            UniformData::FloatVec(3, arr) => {
                if arr.len() % 3 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::Uniform3fv(self.handle, arr.len() as i32 / 3, arr.as_ptr()); }
            },
            UniformData::FloatVec(4, arr) => {
                if arr.len() % 4 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::Uniform4fv(self.handle, arr.len() as i32 / 4, arr.as_ptr()); }
            },

            UniformData::IntVec(1, arr) => {
                unsafe { gl::Uniform1iv(self.handle, arr.len() as i32, arr.as_ptr()); }
            },
            UniformData::IntVec(2, arr) => {
                if arr.len() % 2 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::Uniform2iv(self.handle, arr.len() as i32 / 2, arr.as_ptr()); }
            },
            UniformData::IntVec(3, arr) => {
                if arr.len() % 3 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::Uniform3iv(self.handle, arr.len() as i32 / 3, arr.as_ptr()); }
            },
            UniformData::IntVec(4, arr) => {
                if arr.len() % 4 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::Uniform4iv(self.handle, arr.len() as i32 / 4, arr.as_ptr()); }
            },

            UniformData::UintVec(1, arr) => {
                unsafe { gl::Uniform1uiv(self.handle, arr.len() as i32, arr.as_ptr()); }
            },
            UniformData::UintVec(2, arr) => {
                if arr.len() % 2 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::Uniform2uiv(self.handle, arr.len() as i32 / 2, arr.as_ptr()); }
            },
            UniformData::UintVec(3, arr) => {
                if arr.len() % 3 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::Uniform3uiv(self.handle, arr.len() as i32 / 3, arr.as_ptr()); }
            },
            UniformData::UintVec(4, arr) => {
                if arr.len() % 4 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::Uniform4uiv(self.handle, arr.len() as i32 / 4, arr.as_ptr()); }
            },

            UniformData::FloatMat(2, transpose, arr) => {
                if arr.len() % 4 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix2fv(self.handle, arr.len() as i32 / 4, transpose as u8, arr.as_ptr()); }
            },
            UniformData::FloatMat(3, transpose, arr) => {
                if arr.len() % 9 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix3fv(self.handle, arr.len() as i32 / 9, transpose as u8, arr.as_ptr()); }
            },
            UniformData::FloatMat(4, transpose, arr) => {
                if arr.len() % 16 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix4fv(self.handle, arr.len() as i32 / 16, transpose as u8, arr.as_ptr()); }
            },

            UniformData::FloatMatNxM(2, 2, transpose, arr) => {
                if arr.len() % 4 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix2fv(self.handle, arr.len() as i32 / 4, transpose as u8, arr.as_ptr()); }
            },
            UniformData::FloatMatNxM(2, 3, transpose, arr) => {
                if arr.len() % 6 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix2x3fv(self.handle, arr.len() as i32 / 6, transpose as u8, arr.as_ptr()); }
            },
            UniformData::FloatMatNxM(2, 4, transpose, arr) => {
                if arr.len() % 8 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix2x4fv(self.handle, arr.len() as i32 / 8, transpose as u8, arr.as_ptr()); }
            },

            UniformData::FloatMatNxM(3, 2, transpose, arr) => {
                if arr.len() % 6 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix3x2fv(self.handle, arr.len() as i32 / 6, transpose as u8, arr.as_ptr()); }
            },
            UniformData::FloatMatNxM(3, 3, transpose, arr) => {
                if arr.len() % 9 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix3fv(self.handle, arr.len() as i32 / 9, transpose as u8, arr.as_ptr()); }
            },
            UniformData::FloatMatNxM(3, 4, transpose, arr) => {
                if arr.len() % 12 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix3x4fv(self.handle, arr.len() as i32 / 12, transpose as u8, arr.as_ptr()); }
            },

            UniformData::FloatMatNxM(4, 2, transpose, arr) => {
                if arr.len() % 8 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix4x2fv(self.handle, arr.len() as i32 / 8, transpose as u8, arr.as_ptr()); }
            },
            UniformData::FloatMatNxM(4, 3, transpose, arr) => {
                if arr.len() % 12 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix4x3fv(self.handle, arr.len() as i32 / 12, transpose as u8, arr.as_ptr()); }
            },
            UniformData::FloatMatNxM(4, 4, transpose, arr) => {
                if arr.len() % 16 != 0 { panic!(ERR_ARRAY_SIZE); }
                unsafe { gl::UniformMatrix4fv(self.handle, arr.len() as i32 / 16, transpose as u8, arr.as_ptr()); }
            },

            _ => { panic!("invalid data format"); }
        }
    }
}

impl Program {
    pub fn get_uniform_loc(&self, name: &str) -> Uniform {
        unsafe {
            let loc = gl::GetUniformLocation(self.handle(), CString::new(name).unwrap().as_ptr());

            return Uniform {
                handle: loc,
            };
        }
    }
}

static ERR_ARRAY_SIZE: &'static str = "invalid array size";

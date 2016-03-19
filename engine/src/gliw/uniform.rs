extern crate gl;

use gliw::Program;

use std::ffi::CString;

pub enum UniformData<'a> {
    /// tuple `Float1(v0)`
    Float1(f32),
    /// tuple `Float2(v0, v1)`
    Float2(f32, f32),
    /// tuple `Float3(v0, v1, v2)`
    Float3(f32, f32, f32),
    /// tuple `Float4(v0, v1, v2, v3)`
    Float4(f32, f32, f32, f32),

    /// tuple `Int1(v0)`
    Int1(i32),
    /// tuple `Int2(v0, v1)`
    Int2(i32, i32),
    /// tuple `Int3(v0, v1, v2)`
    Int3(i32, i32, i32),
    /// tuple `Int4(v0, v1, v2, v3)`
    Int4(i32, i32, i32, i32),

    /// tuple `Uint1(v0)`
    Uint1(u32),
    /// tuple `Uint2(v0, v1)`
    Uint2(u32, u32),
    /// tuple `Uint3(v0, v1, v2)`
    Uint3(u32, u32, u32),
    /// tuple `Uint4(v0, v1, v2, v3)`
    Uint4(u32, u32, u32, u32),

    /// tuple `FloatVec(size, slice)` <br>
    /// `size` can be 1, 2, 3 or 4 <br>
    /// `slice` must be a `&[f32]` with lenght multiple of `size`
    FloatVec(i32, &'a [f32]),

    /// tuple `IntVec(size, slice)` <br>
    /// `size` can be 1, 2, 3 or 4 <br>
    /// `slice` must be a `&[i32]` with lenght multiple of `size`
    IntVec(i32, &'a [i32]),

    /// tuple `UintVec(size, slice)` <br>
    /// `size` can be 1, 2, 3 or 4 <br>
    /// `slice` must be a `&[u32]` with lenght multiple of `size`
    UintVec(i32, &'a [u32]),

    /// tuple `FloatMat(size, transpose, slice)` - an NxN matrix <br>
    /// `size` can be 2, 3 or 4 <br>
    /// `transpose` spceifies whether the matrix should be passed to the shader as is or transposed <br>
    /// `slice` must be a `&[f32]` with lenght muptiple of `size * size`
    FloatMat(i32, bool, &'a [f32]),

    /// tuple `FloatMatNxM(n, m, transpose, slice)` - an NxM matrix <br>
    /// `n` and `m` can be 2, 3 or 4 <br>
    /// `transpose` spceifies whether the matrix should be passed to the shader as is or transposed <br>
    /// `slice` must be a `&[f32]` with lenght muptiple of `n * m` <br>
    FloatMatNxM(i32, i32, bool, &'a [f32]),
}

/// Wrapper for an OpenGL uniform location
///
/// Note that this class does not give static guarantees that an actual attribute exists.
/// This is because the return value of `glGetUniformLocation` is ambiguous - a value of `-1`
/// can mean that either no variable with the given name exists, or that it exists but is unused,
/// so it has been optimized out by the driver.
pub struct Uniform<'a> {
    handle: i32,
    prog: &'a Program,
}

impl<'a> Uniform<'a> {
    /// Wrapper for `glUniform*` and `glUniformMatrix*`
    ///
    /// Sets the value of the uniform variable.
    ///
    /// # Panics
    ///
    /// if one of `FloatVec`, `IntVec`, `UintVec`, `FloatMat` or `FloatMatNxM` is passed for `data` and the lenght of
    /// the slice is not a multiple of the size of the type of the uniform variable <br>
    /// if an invalid size is passed using `FloatVec`, IntVec`, UintVec`, `FloatMat` or `FloatMatNxM` <br>
    /// if the specified type for `data` does not match the type of the uniform variable
    ///
    pub fn value<'b> (&'b self, data: UniformData<'b>) {
        // Clear all previous error
        // this is an unintended side effect, but i don't see a way around it
        // TODO: we could instead query the type using `glGetUniformIndices` and `glGetActiveUniform`
        unsafe {
            while gl::GetError() != gl::NO_ERROR {
            }
        }

        self.prog.bind();

        macro_rules! set_vec_uniform {
            ($fun:expr, 1, $arr:expr) => (
                unsafe {
                    $fun(self.handle, $arr.len() as i32, $arr.as_ptr());
                }
            );
            ($fun:expr, $cnt:expr, $arr:expr) => (
                unsafe {
                    if $arr.len() % $cnt != 0 {
                        panic!(ERR_ARRAY_SIZE);
                    }
                    $fun(self.handle, ($arr.len() / $cnt) as i32, $arr.as_ptr());
                }
            );
        }

        macro_rules! set_mat_uniform {
            ($fun:expr, $n:expr, $m:expr, $transpose:expr, $arr:expr) => (
                unsafe {
                    let dim: usize = $n * $m;
                    if $arr.len() % dim != 0 {
                        panic!(ERR_ARRAY_SIZE);
                    }
                    $fun(self.handle, ($arr.len() / dim) as i32, $transpose as u8, $arr.as_ptr());
                }
            );
        }

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

            UniformData::FloatVec(1, arr) => set_vec_uniform!(gl::Uniform1fv, 1, arr),
            UniformData::FloatVec(2, arr) => set_vec_uniform!(gl::Uniform2fv, 2, arr),
            UniformData::FloatVec(3, arr) => set_vec_uniform!(gl::Uniform3fv, 3, arr),
            UniformData::FloatVec(4, arr) => set_vec_uniform!(gl::Uniform4fv, 4, arr),

            UniformData::IntVec(1, arr) => set_vec_uniform!(gl::Uniform1iv, 1, arr),
            UniformData::IntVec(2, arr) => set_vec_uniform!(gl::Uniform2iv, 2, arr),
            UniformData::IntVec(3, arr) => set_vec_uniform!(gl::Uniform3iv, 3, arr),
            UniformData::IntVec(4, arr) => set_vec_uniform!(gl::Uniform4iv, 4, arr),

            UniformData::UintVec(1, arr) => set_vec_uniform!(gl::Uniform1uiv, 1, arr),
            UniformData::UintVec(2, arr) => set_vec_uniform!(gl::Uniform2uiv, 2, arr),
            UniformData::UintVec(3, arr) => set_vec_uniform!(gl::Uniform3uiv, 3, arr),
            UniformData::UintVec(4, arr) => set_vec_uniform!(gl::Uniform4uiv, 4, arr),

            UniformData::FloatMat(2, transpose, arr) => set_mat_uniform!(gl::UniformMatrix2fv, 2, 2, transpose, arr),
            UniformData::FloatMat(3, transpose, arr) => set_mat_uniform!(gl::UniformMatrix3fv, 3, 3, transpose, arr),
            UniformData::FloatMat(4, transpose, arr) => set_mat_uniform!(gl::UniformMatrix4fv, 4, 4, transpose, arr),

            UniformData::FloatMatNxM(2, 2, transpose, arr) => set_mat_uniform!(gl::UniformMatrix2fv, 2, 2, transpose, arr),
            UniformData::FloatMatNxM(2, 3, transpose, arr) => set_mat_uniform!(gl::UniformMatrix2x3fv, 2, 3, transpose, arr),
            UniformData::FloatMatNxM(2, 4, transpose, arr) => set_mat_uniform!(gl::UniformMatrix2x4fv, 2, 4, transpose, arr),

            UniformData::FloatMatNxM(3, 2, transpose, arr) => set_mat_uniform!(gl::UniformMatrix3x2fv, 3, 2, transpose, arr),
            UniformData::FloatMatNxM(3, 3, transpose, arr) => set_mat_uniform!(gl::UniformMatrix3fv, 3, 3, transpose, arr),
            UniformData::FloatMatNxM(3, 4, transpose, arr) => set_mat_uniform!(gl::UniformMatrix3x4fv, 3, 4, transpose, arr),

            UniformData::FloatMatNxM(4, 2, transpose, arr) => set_mat_uniform!(gl::UniformMatrix4x2fv, 4, 2, transpose, arr),
            UniformData::FloatMatNxM(4, 3, transpose, arr) => set_mat_uniform!(gl::UniformMatrix4x3fv, 4, 3, transpose, arr),
            UniformData::FloatMatNxM(4, 4, transpose, arr) => set_mat_uniform!(gl::UniformMatrix4fv, 4, 4, transpose, arr),

            _ => { panic!(ERR_DATA_FORMAT); }
        }

        unsafe {
            if gl::GetError() == gl::INVALID_OPERATION {
                panic!(ERR_TYPE_MISSMATCH);
            }
        }
    }

    pub fn handle(&self) -> i32 {
        return self.handle;
    }
}

impl Program {
    /// Wrapper for `glGetUniformLocation`
    pub fn get_uniform_loc<'a> (&'a self, name: &str) -> Uniform<'a> {
        unsafe {
            let loc = gl::GetUniformLocation(self.handle(), CString::new(name).unwrap().as_ptr());

            return Uniform {
                handle: loc,
                prog: self,
            };
        }
    }
}

const ERR_ARRAY_SIZE: &'static str = "Invalid array size - the lenght of the slice must be a multiple of the size of the type";
const ERR_DATA_FORMAT: &'static str = "Invalid data format";
const ERR_TYPE_MISSMATCH: &'static str = "Specified data does not match the type of the uniform variable as declared in the shader";

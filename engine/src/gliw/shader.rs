extern crate gl;

use std::ffi::CString;
use std::fs::File;
use std::ptr;

use std::io::Read;

#[repr(u32)]
pub enum ShaderType {
    Compute         = gl::COMPUTE_SHADER,
    Vertex          = gl::VERTEX_SHADER,
    TessControl     = gl::TESS_CONTROL_SHADER,
    TessEvaluation  = gl::TESS_EVALUATION_SHADER,
    Geometry        = gl::GEOMETRY_SHADER,
    Fragment        = gl::FRAGMENT_SHADER,
}

/// Wrapper for a compiled OpenGL shader object
pub struct Shader {
    handle: u32,
}

impl Shader {
    pub fn new (shader_type: ShaderType, shader_code: &str) -> Result<Shader, String> {
        unsafe {
            let content = CString::new(shader_code).unwrap();
            let content_ptr = content.as_ptr();

            let shader = gl::CreateShader(shader_type as u32);
            gl::ShaderSource(shader, 1, &content_ptr, ptr::null());
            gl::CompileShader(shader);

            let mut status: i32 = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
            if status != (gl::TRUE as i32) {
                let mut log_size: i32 = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_size);

                let buff = CString::from_vec_unchecked(vec![0u8; log_size as usize]);
                gl::GetShaderInfoLog(shader, log_size, 0 as *mut i32, buff.as_ptr() as *mut i8);

                gl::DeleteShader(shader);
                return Err(buff.to_str().unwrap().to_string());
            }

            return Ok(Shader{
                handle: shader
            });
        }
    }

    pub fn from_file (shader_type: ShaderType, filename: &str) -> Result<Shader, String> {
        let mut content = String::new();
        match File::open(filename) {
            Ok(mut file) => { file.read_to_string(&mut content).unwrap(); },
            Err(err) => { return Err(format!("{}", err)); }
        }

        return Self::new(shader_type, &content);
    }

    pub fn handle (&self) -> u32 {
        return self.handle;
    }
}

impl Drop for Shader {
    fn drop (&mut self) {
        unsafe { gl::DeleteShader(self.handle); }
    }
}

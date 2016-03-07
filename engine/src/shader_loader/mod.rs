//! Utility functions for loading shaders

extern crate gl;

use std::ptr;
use std::ffi::CString;
use std::fs::File;
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

/// Compiles shader code from string into a shader object
pub fn compile_shader(shader_code: String, shader_type: ShaderType) -> Result<u32, String> {
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

        return Ok(shader);
    }
}

/// A builder class for linking already compiled shaders into a program
pub struct ProgramLinkBuilder {
    cs: u32,
    vs: u32,
    tcs: u32,
    tes: u32,
    gs: u32,
    fs: u32,
}

impl ProgramLinkBuilder {
    pub fn new() -> ProgramLinkBuilder {
        ProgramLinkBuilder {
            cs: 0,
            vs: 0,
            tcs: 0,
            tes: 0,
            gs: 0,
            fs: 0,
        }
    }

    pub fn attach_cs (&mut self, name: u32) {
        self.cs = name;
    }

    pub fn attach_vs (&mut self, name: u32) {
        self.vs = name;
    }

    pub fn attach_tcs (&mut self, name: u32) {
        self.tcs = name;
    }

    pub fn attach_tes (&mut self, name: u32) {
        self.tes = name;
    }

    pub fn attach_gs (&mut self, name: u32) {
        self.gs = name;
    }

    pub fn attach_fs (&mut self, name: u32) {
        self.fs = name;
    }

    /// Links a program object using the attached shaders
    pub fn link(&self) -> Result<u32, String> {
        unsafe {
            let prog = gl::CreateProgram();

            if self.cs != 0 { gl::AttachShader(prog, self.cs); }
            if self.vs != 0 { gl::AttachShader(prog, self.vs); }
            if self.tcs != 0 { gl::AttachShader(prog, self.tcs); }
            if self.tes != 0 { gl::AttachShader(prog, self.tes); }
            if self.gs != 0 { gl::AttachShader(prog, self.gs); }
            if self.fs != 0 { gl::AttachShader(prog, self.fs); }

            gl::LinkProgram(prog);

            let mut status: i32 = 0;
            gl::GetProgramiv(prog, gl::LINK_STATUS, &mut status);
            if status != (gl::TRUE as i32) {
                let mut log_size: i32 = 0;
                gl::GetProgramiv(prog, gl::INFO_LOG_LENGTH, &mut log_size);

                let buff = CString::from_vec_unchecked(vec![0u8; log_size as usize]);
                gl::GetProgramInfoLog(prog, log_size, 0 as *mut i32, buff.as_ptr() as *mut i8);

                gl::DeleteProgram(prog);
                return Err(buff.to_str().unwrap().to_string());
            }

            if self.cs != 0 { gl::DetachShader(prog, self.cs); }
            if self.vs != 0 { gl::DetachShader(prog, self.vs); }
            if self.tcs != 0 { gl::DetachShader(prog, self.tcs); }
            if self.tes != 0 { gl::DetachShader(prog, self.tes); }
            if self.gs != 0 { gl::DetachShader(prog, self.gs); }
            if self.fs != 0 { gl::DetachShader(prog, self.fs); }

            return Ok(prog);
        }
    }
}

/// A builder class for loading shaders from files
pub struct ProgramBuilder<'a> {
    cs_path: Option<&'a str>,
    vs_path: Option<&'a str>,
    tcs_path: Option<&'a str>,
    tes_path: Option<&'a str>,
    gs_path: Option<&'a str>,
    fs_path: Option<&'a str>,
}

impl<'a> ProgramBuilder<'a> {
    pub fn new() -> ProgramBuilder<'a> {
        ProgramBuilder {
            cs_path: None,
            vs_path: None,
            tcs_path: None,
            tes_path: None,
            gs_path: None,
            fs_path: None,
        }
    }

    pub fn cs_path(&mut self, path: &'a str) -> &'a mut ProgramBuilder {
        self.cs_path = Some(path);
        self
    }

    pub fn vs_path(&mut self, path: &'a str) -> &'a mut ProgramBuilder {
        self.vs_path = Some(path);
        self
    }

    pub fn tcs_path(&mut self, path: &'a str) -> &'a mut ProgramBuilder {
        self.tcs_path = Some(path);
        self
    }

    pub fn tes_path(&mut self, path: &'a str) -> &'a mut ProgramBuilder {
        self.tes_path = Some(path);
        self
    }

    pub fn gs_path(&mut self, path: &'a str) -> &'a mut ProgramBuilder {
        self.gs_path = Some(path);
        self
    }

    pub fn fs_path(&mut self, path: &'a str) -> &'a mut ProgramBuilder {
        self.fs_path = Some(path);
        self
    }


    /// Consumes the builder to produce a program object
    /// Loads the shader code from files, compiles it and links the shaders in a program
    pub fn compile(&self) -> Result<u32, String> {

        fn helper(filename: &str, shader_type: ShaderType) -> Result<u32, String> {
            let mut content = String::new();
            match File::open(filename) {
                Ok(mut file) => { file.read_to_string(&mut content).unwrap(); },
                Err(err) => { return Err(format!("{}", err)); }
            }
            compile_shader(content, shader_type)
        }

        let mut cs: u32 = 0;
        let mut vs: u32 = 0;
        let mut tcs: u32 = 0;
        let mut tes: u32 = 0;
        let mut gs: u32 = 0;
        let mut fs: u32 = 0;

        if self.cs_path.is_some() {
            let filename = self.cs_path.unwrap();
            match helper(filename, ShaderType::Compute) {
                Ok(res) => { cs = res; }
                Err(err) => { return Err(format!("failed compilation of compute shader {}:\n{}", filename, err)); }
            }
        }

        if self.vs_path.is_some() {
            let filename = self.vs_path.unwrap();
            match helper(filename, ShaderType::Vertex) {
                Ok(res) => { vs = res; }
                Err(err) => { return Err(format!("failed compilation of vertex shader {}:\n{}", filename, err)); }
            }
        }

        if self.tcs_path.is_some() {
            let filename = self.tcs_path.unwrap();
            match helper(filename, ShaderType::TessControl) {
                Ok(res) => { tcs = res; }
                Err(err) => { return Err(format!("failed compilation of tesselation control shader {}:\n{}", filename, err)); }
            }
        }

        if self.tes_path.is_some() {
            let filename = self.tes_path.unwrap();
            match helper(filename, ShaderType::TessEvaluation) {
                Ok(res) => { tes = res; }
                Err(err) => { return Err(format!("failed compilation of tesselation evaluation shader {}:\n{}", filename, err)); }
            }
        }

        if self.gs_path.is_some() {
            let filename = self.gs_path.unwrap();
            match helper(filename, ShaderType::Geometry) {
                Ok(res) => { gs = res; }
                Err(err) => { return Err(format!("failed compilation of geometry shader {}:\n{}", filename, err)); }
            }
        }

        if self.fs_path.is_some() {
            let filename = self.fs_path.unwrap();
            match helper(filename, ShaderType::Fragment) {
                Ok(res) => { fs = res; }
                Err(err) => { return Err(format!("failed compilation of fragment shader {}:\n{}", filename, err)); }
            }
        }

        let prog = ProgramLinkBuilder{
            cs: cs,
            vs: vs,
            tcs: tcs,
            tes: tes,
            gs: gs,
            fs: fs,
        }.link().unwrap();

        unsafe {
            gl::DeleteShader(cs);
            gl::DeleteShader(vs);
            gl::DeleteShader(tcs);
            gl::DeleteShader(tes);
            gl::DeleteShader(gs);
            gl::DeleteShader(fs);
        }
        Ok(prog)
    }
}

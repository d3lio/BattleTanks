extern crate gl;

use gliw::{Shader, ShaderType};

use std::ffi::CString;
use std::ptr;

/// Wrapper for a linked OpenGL program
///
/// Created using `ProgramBuilder` or `ProgramFromFileBuilder`
pub struct Program {
    handle: u32,
}

impl Program {
    /// Wrapper for `glUseProgram`
    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.handle); }
    }

    /// Get the underlying OpenGL handle
    pub fn handle(&self) -> u32 {
        return self.handle;
    }
}

impl Drop for Program {
    fn drop (&mut self) {
        unsafe { gl::DeleteProgram(self.handle); }
    }
}

/// A builder class for linking a program using compiled shaders
///
/// # Example
///
/// ```no_run
/// # use engine::gliw::{Shader, ShaderType, ProgramBuilder};
/// let compiled_vs = Shader::new(ShaderType::Vertex, "<code>").unwrap();
/// let compiled_fs = Shader::new(ShaderType::Fragment, "<code>").unwrap();
///
/// let prog = ProgramBuilder::new()
///     .attach_vs(&compiled_vs)
///     .attach_fs(&compiled_fs)
///     .link()
///     .unwrap();
/// ```
pub struct ProgramBuilder<'a> {
    cs: Option<&'a Shader>,
    vs: Option<&'a Shader>,
    tcs: Option<&'a Shader>,
    tes: Option<&'a Shader>,
    gs: Option<&'a Shader>,
    fs: Option<&'a Shader>,
}

impl<'a> ProgramBuilder<'a> {
    pub fn new() -> ProgramBuilder<'a> {
        return ProgramBuilder {
            cs: None,
            vs: None,
            tcs: None,
            tes: None,
            gs: None,
            fs: None,
        }
    }

    /// Set compute shader to attach
    pub fn attach_cs (&mut self, shader: &'a Shader) -> &'a mut ProgramBuilder {
        // TODO: check if shader is compute shader?
        self.cs = Some(shader);
        return self;
    }

    /// Set vertex shader to attach
    pub fn attach_vs (&mut self, shader: &'a Shader) -> &'a mut ProgramBuilder {
        self.vs = Some(shader);
        return self;
    }

    /// Set tesselation control shader to attach
    pub fn attach_tcs (&mut self, shader: &'a Shader) -> &'a mut ProgramBuilder {
        self.tcs = Some(shader);
        return self;
    }

    /// Set tesselation evaluation shader to attach
    pub fn attach_tes (&mut self, shader: &'a Shader) -> &'a mut ProgramBuilder {
        self.tes = Some(shader);
        return self;
    }

    /// Set geometry shader to attach
    pub fn attach_gs (&mut self, shader: &'a Shader) -> &'a mut ProgramBuilder {
        self.gs = Some(shader);
        return self;
    }

    /// Set fragment shader to attach
    pub fn attach_fs (&mut self, shader: &'a Shader) -> &'a mut ProgramBuilder {
        self.fs = Some(shader);
        return self;
    }

    /// Links a program object using the attached shaders
    pub fn link(&self) -> Result<Program, String> {
        unsafe {
            let prog = gl::CreateProgram();

            if let Some(shader) = self.cs { gl::AttachShader(prog, shader.handle()); }
            if let Some(shader) = self.vs { gl::AttachShader(prog, shader.handle()); }
            if let Some(shader) = self.tcs { gl::AttachShader(prog, shader.handle()); }
            if let Some(shader) = self.tes { gl::AttachShader(prog, shader.handle()); }
            if let Some(shader) = self.gs { gl::AttachShader(prog, shader.handle()); }
            if let Some(shader) = self.fs { gl::AttachShader(prog, shader.handle()); }

            gl::LinkProgram(prog);

            let mut status: i32 = 0;
            gl::GetProgramiv(prog, gl::LINK_STATUS, &mut status);
            if status != (gl::TRUE as i32) {
                let mut log_size: i32 = 0;
                gl::GetProgramiv(prog, gl::INFO_LOG_LENGTH, &mut log_size);

                let buff = CString::from_vec_unchecked(vec![0u8; log_size as usize]);
                gl::GetProgramInfoLog(prog, log_size, ptr::null_mut(), buff.as_ptr() as *mut _);

                gl::DeleteProgram(prog);
                return Err(buff.to_str().unwrap().to_string());
            }

            if let Some(shader) = self.cs { gl::DetachShader(prog, shader.handle()); }
            if let Some(shader) = self.vs { gl::DetachShader(prog, shader.handle()); }
            if let Some(shader) = self.tcs { gl::DetachShader(prog, shader.handle()); }
            if let Some(shader) = self.tes { gl::DetachShader(prog, shader.handle()); }
            if let Some(shader) = self.gs { gl::DetachShader(prog, shader.handle()); }
            if let Some(shader) = self.fs { gl::DetachShader(prog, shader.handle()); }

            return Ok(Program{
                handle: prog,
            });
        }
    }
}

/// An utility builder class for compiling and linking a program using shader code from files
///
/// # Example
/// ```no_run
/// use engine::gliw::ProgramFromFileBuilder;
///
/// let prog = ProgramFromFileBuilder::new()
///     .vs_path("file.vs")
///     .fs_path("file.fs")
///     .compile()
///     .unwrap();
/// ```
pub struct ProgramFromFileBuilder<'a> {
    cs_path: Option<&'a str>,
    vs_path: Option<&'a str>,
    tcs_path: Option<&'a str>,
    tes_path: Option<&'a str>,
    gs_path: Option<&'a str>,
    fs_path: Option<&'a str>,
}

impl<'a> ProgramFromFileBuilder<'a> {
    pub fn new() -> ProgramFromFileBuilder<'a> {
        ProgramFromFileBuilder {
            cs_path: None,
            vs_path: None,
            tcs_path: None,
            tes_path: None,
            gs_path: None,
            fs_path: None,
        }
    }

    /// Set the file containing compute shader code
    pub fn cs_path(&mut self, path: &'a str) -> &'a mut ProgramFromFileBuilder {
        self.cs_path = Some(path);
        return self;
    }

    /// Set the file containing vertex shader code
    pub fn vs_path(&mut self, path: &'a str) -> &'a mut ProgramFromFileBuilder {
        self.vs_path = Some(path);
        return self;
    }

    /// Set the file containing tesselation control shader code
    pub fn tcs_path(&mut self, path: &'a str) -> &'a mut ProgramFromFileBuilder {
        self.tcs_path = Some(path);
        return self;
    }

    /// Set the file containing tesselation evaluation shader code
    pub fn tes_path(&mut self, path: &'a str) -> &'a mut ProgramFromFileBuilder {
        self.tes_path = Some(path);
        return self;
    }

    /// Set the file containing geometry shader code
    pub fn gs_path(&mut self, path: &'a str) -> &'a mut ProgramFromFileBuilder {
        self.gs_path = Some(path);
        return self;
    }

    /// Set the file containing fragment shader code
    pub fn fs_path(&mut self, path: &'a str) -> &'a mut ProgramFromFileBuilder {
        self.fs_path = Some(path);
        return self;
    }

    /// Compiles the provided shaders and links them into a program
    pub fn compile(&self) -> Result<Program, String> {
        let cs: Shader;
        let vs: Shader;
        let tcs: Shader;
        let tes: Shader;
        let gs: Shader;
        let fs: Shader;

        let mut prog_builder = ProgramBuilder::new();

        if let Some(filename) = self.cs_path {
            match Shader::from_file(ShaderType::Compute, filename) {
                Ok(res) => { cs = res; prog_builder.cs = Some(&cs); },
                Err(err) => { return Err(format!("failed compilation of compute shader {}:\n{}", filename, err)); }
            }
        }

        if let Some(filename) = self.vs_path {
            match Shader::from_file(ShaderType::Vertex, filename) {
                Ok(res) => { vs = res; prog_builder.vs = Some(&vs); },
                Err(err) => { return Err(format!("failed compilation of vertex shader {}:\n{}", filename, err)); }
            }
        }

        if let Some(filename) = self.tcs_path {
            match Shader::from_file(ShaderType::TessControl, filename) {
                Ok(res) => { tcs = res; prog_builder.tcs = Some(&tcs); },
                Err(err) => { return Err(format!("failed compilation of tesselation control shader {}:\n{}", filename, err)); }
            }
        }

        if let Some(filename) = self.tes_path {
            match Shader::from_file(ShaderType::TessEvaluation, filename) {
                Ok(res) => { tes = res; prog_builder.tes = Some(&tes); },
                Err(err) => { return Err(format!("failed compilation of tesselation evaluation shader {}:\n{}", filename, err)); }
            }
        }

        if let Some(filename) = self.gs_path {
            match Shader::from_file(ShaderType::Geometry, filename) {
                Ok(res) => { gs = res; prog_builder.gs = Some(&gs); },
                Err(err) => { return Err(format!("failed compilation of geometry shader {}:\n{}", filename, err)); }
            }
        }

        if let Some(filename) = self.fs_path {
            match Shader::from_file(ShaderType::Fragment, filename) {
                Ok(res) => { fs = res; prog_builder.fs = Some(&fs); },
                Err(err) => { return Err(format!("failed compilation of framgent shader {}:\n{}", filename, err)); }
            }
        }

        return prog_builder.link();
    }
}

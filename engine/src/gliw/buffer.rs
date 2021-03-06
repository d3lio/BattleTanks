extern crate gl;

use gliw::error;

use std::mem;
use std::os::raw::c_void;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BufferType {
    Array               = gl::ARRAY_BUFFER,
    AtomicCounter       = gl::ATOMIC_COUNTER_BUFFER,
    CopyRead            = gl::COPY_READ_BUFFER,
    CopyWrite           = gl::COPY_WRITE_BUFFER,
    DrawIndirect        = gl::DRAW_INDIRECT_BUFFER,
    DispatchIndirect    = gl::DISPATCH_INDIRECT_BUFFER,
    ElementArray        = gl::ELEMENT_ARRAY_BUFFER,
    PixelPack           = gl::PIXEL_PACK_BUFFER,
    PixelUnpack         = gl::PIXEL_UNPACK_BUFFER,
    Query               = gl::QUERY_BUFFER,
    ShaderStorage       = gl::SHADER_STORAGE_BUFFER,
    Texture             = gl::TEXTURE_BUFFER,
    TransformFeedback   = gl::TRANSFORM_FEEDBACK_BUFFER,
    Uniform             = gl::UNIFORM_BUFFER,
}

#[repr(u32)]
pub enum BufferUsagePattern {
    StreamDraw      = gl::STREAM_DRAW,
    StreamRead      = gl::STREAM_READ,
    StreamCopy      = gl::STREAM_COPY,
    StaticDraw      = gl::STATIC_DRAW,
    StaticRead      = gl::STATIC_READ,
    StaticCopy      = gl::STATIC_COPY,
    DynamicDraw     = gl::DYNAMIC_DRAW,
    DynamicRead     = gl::DYNAMIC_READ,
    DynamicCopy     = gl::DYNAMIC_COPY,
}

/// Wrapper for OpenGL Buffer Object.
///
/// # Examples
///
/// Seperate creation:
///
/// ```no_run
/// # use engine::gliw::{Buffer, BufferType, BufferUsagePattern};
/// # let VERTEX_DATA: [f32; 9] = [-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0,  1.0, 0.0];
/// let vbo = Buffer::new(BufferType::Array);
/// vbo.buffer_data(&VERTEX_DATA, BufferUsagePattern::StaticDraw);
/// ```
///
/// Combined creation:
///
/// ```no_run
/// # use engine::gliw::{Buffer, BufferType, BufferUsagePattern};
/// # let VERTEX_DATA: [f32; 9] = [-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0,  1.0, 0.0];
/// let vbo = Buffer::from_data(
///     &VERTEX_DATA,
///     BufferType::Array,
///     BufferUsagePattern::StaticDraw);
/// ```
///
/// # References
/// * [Buffer Object](https://www.opengl.org/wiki/Buffer_Object)
/// * [Vertex Buffer Object](https://www.opengl.org/wiki/Vertex_Specification#Vertex_Buffer_Object)
pub struct Buffer {
    handle: u32,
    buf_type: BufferType
}

impl Buffer {
    /// Generate a buffer and set it's type (target) for safe future gl function calls.
    pub fn new(buf_type: BufferType) -> Buffer {
        let mut vbo = Buffer {
            handle: 0,
            buf_type: buf_type
        };

        unsafe { gl::GenBuffers(1, &mut vbo.handle as *mut u32); }

        return vbo;
    }

    /// Combines new and bind for convenience.
    pub fn from_data<T>(vertices: &[T], buf_type: BufferType, usage: BufferUsagePattern) -> Buffer {
        let vbo = Buffer::new(buf_type);
        vbo.buffer_data(vertices, usage);

        return vbo;
    }

    /// Wrapper for `glBindBuffer`.
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.buf_type as u32, self.handle); }
    }

    /// The engine's equivalent to `glBufferData`.
    ///
    /// Binds self internally.
    pub fn buffer_data<T>(&self, vertices: &[T], usage: BufferUsagePattern) {
        self.bind();
        unsafe {
            gl::BufferData(
                self.buf_type as u32,
                (vertices.len() * mem::size_of::<T>()) as isize,
                vertices.as_ptr() as *const c_void,
                usage as u32);
            if gl::GetError() == error::GL_OUT_OF_MEMORY.num {
                panic!(error::GL_OUT_OF_MEMORY.msg);
            }
        }
    }

    /// Get the buffer's type (target).
    pub fn buf_type(&self) -> BufferType {
        return self.buf_type;
    }

    /// Get the underlying OpenGL handle.
    pub fn handle(&self) -> u32 {
        return self.handle;
    }
}

impl Drop for Buffer {
    fn drop (&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.handle); }
    }
}

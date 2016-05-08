extern crate gl;
extern crate cgmath;

use self::cgmath::{
    VectorSpace, EuclideanSpace,
    Point3, Vector3, Vector4, Matrix4, Quaternion
};

use gliw::{
    Buffer, BufferType, BufferUsagePattern,
    Program, ProgramBuilder, Uniform,
    Shader, ShaderType,
    UniformData,
    Vao,
    VertexAttrib, AttribFloatFormat
};

use super::Entity;

use core::{Camera, Renderable};

use math::RotMat;

use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::ptr;
use std::mem;

/// A general purpose cuboid entity.
#[allow(dead_code)]
pub struct Cuboid {
    entity: Entity,
    dimensions: Vector3<f32>,
    color: Vector4<f32>,
    priority: u32,

    vao: Vao,
    vbo: Buffer,            // FIXME: should be static
    ebo: Buffer,            // FIXME: should be static
    program: Rc<Program>
}

impl Cuboid {
    /// Creates a new cuboid from given center, dimensions and color.
    pub fn new(center: Point3<f32>, dimensions: Vector3<f32>, color: Vector4<f32>) -> Cuboid {
        // FIXME: should be static
        let program = ProgramBuilder::new()
            .attach_vs(&Shader::new(ShaderType::Vertex, VS_SRC).unwrap())
            .attach_fs(&Shader::new(ShaderType::Fragment, FS_SRC).unwrap())
            .link()
            .unwrap();

        let vao = Vao::new();

        let vbo = Buffer::from_data(
            &VERTICES,
            BufferType::Array,
            BufferUsagePattern::StaticDraw);

        let ebo = Buffer::from_data(
            &ELEMENTS,
            BufferType::ElementArray,
            BufferUsagePattern::StaticDraw);

        let va = VertexAttrib::new(0);
        va.data_float_format(&vao, &vbo, AttribFloatFormat::Float(3), 0, ptr::null());
        va.enable(&vao);

        return Cuboid {
            entity: Entity::from(center, Quaternion::zero(), 1.0),
            dimensions: dimensions,
            color: color,
            priority: 0,
            vao: vao,
            vbo: vbo,
            ebo: ebo,
            program: program
        };
    }

    /// Get a reference to the cuboid's entity.
    ///
    /// This method is added for explicity. Consider using the deref.
    pub fn entity(&self) -> &Entity {
        return &self.entity;
    }

    /// Get a mutable reference to the cuboid's entity.
    ///
    /// This method is added for explicity. Consider using the deref.
    pub fn entity_mut(&mut self) -> &mut Entity {
        return &mut self.entity;
    }

    /// Set rendering priority.
    pub fn set_priority(&mut self, priority: u32) {
        self.priority = priority;
    }
}

impl Deref for Cuboid {
    type Target = Entity;

    fn deref(&self) -> &Entity {
        return &self.entity;
    }
}

impl DerefMut for Cuboid {
    fn deref_mut(&mut self) -> &mut Entity {
        return &mut self.entity;
    }
}

impl Renderable for Cuboid {
    fn priority(&self) -> u32 {
        return self.priority;
    }

    fn model_matrix(&self) -> Matrix4<f32> {
        let scale_matrix = Matrix4::from_nonuniform_scale(
            self.dimensions.x * self.entity.scale,
            self.dimensions.y * self.entity.scale,
            self.dimensions.z * self.entity.scale);
        let rotation_matrix = Matrix4::from_quat(&self.entity.orientation);
        let translate_matrix = Matrix4::from_translation(self.entity.position.to_vec());

        return translate_matrix * rotation_matrix * scale_matrix;
    }

    fn draw(&self, draw_space: Matrix4<f32>, camera: &Camera) {
        self.vao.bind();
        self.program.bind();

        let mvp_matrix = camera.vp_matrix() * draw_space * self.model_matrix();

        unsafe {
            Uniform::new(&self.program, "cuboid_color").value(UniformData::FloatVec(4,
                &mem::transmute::<Vector4<f32>, [f32; 4]>(self.color)));

            Uniform::new(&self.program, "mvp").value(UniformData::FloatMat(4, false,
                &mem::transmute::<Matrix4<f32>, [f32; 16]>(mvp_matrix)));
        }

        self.ebo.bind();

        unsafe { gl::DrawElements(gl::TRIANGLES, 12*3, gl::UNSIGNED_BYTE, ptr::null()); }
    }
}

const VS_SRC: &'static str = r#"
    #version 330 core

    uniform mat4 mvp;

    layout (location = 0) in vec3 vs_position;

    void main() {
        gl_Position = mvp * vec4(vs_position, 1.0);
    }
"#;

const FS_SRC: &'static str = r#"
    #version 330 core

    uniform vec4 cuboid_color;

    out vec4 color;

    void main() {
        color = cuboid_color;
    }
"#;

static VERTICES: [f32; 8*3] = [
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5, -0.5,
    -0.5,  0.5,  0.5,
     0.5, -0.5, -0.5,
     0.5, -0.5,  0.5,
     0.5,  0.5, -0.5,
     0.5,  0.5,  0.5,
];

static ELEMENTS: [u8; 12*3] = [
    0, 1, 3,
    0, 3, 2,
    5, 4, 6,
    5, 6, 7,
    1, 0, 4,
    1, 4, 5,
    6, 2, 3,
    6, 3, 7,
    4, 0, 2,
    4, 2, 6,
    3, 1, 5,
    3, 5, 7,
];

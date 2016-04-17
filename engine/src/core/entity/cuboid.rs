extern crate gl;
extern crate cgmath;

use self::cgmath::{Point, Point3, Vector3, Vector4, Matrix4, Quaternion, Rotation};

use gliw::{
    Buffer, BufferType, BufferUsagePattern,
    Program, ProgramBuilder,
    Shader, ShaderType,
    UniformData,
    Vao,
    VertexAttrib, AttribFloatFormat
};

use core::{Camera, Renderable, Entity};

use math::RotMat;

use std::ptr;
use std::mem;

/// A general purpose cuboid entity.
#[allow(dead_code)]
pub struct Cuboid {
    center: Point3<f32>,
    dimensions: Vector3<f32>,
    orientation: Quaternion<f32>,
    scale: f32,

    color: Vector4<f32>,

    priority: u32,

    vao: Vao,
    vbo: Buffer,        // FIXME: should be static
    ebo: Buffer,        // FIXME: should be static
    program: Program    // FIXME: should be static
}

impl Cuboid {
    /// Creates a new cuboid from given center, dimensions and color.
    pub fn new(center: Point3<f32>, dimensions: Vector3<f32>, color: Vector4<f32>) -> Cuboid {
        let program = ProgramBuilder::new()
            .attach_vs(&Shader::new(ShaderType::Vertex, VS_SRC).unwrap())
            .attach_fs(&Shader::new(ShaderType::Fragment, FS_SRC).unwrap())
            .link()
            .unwrap();

        let vao = Vao::new();
        vao.bind();

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
            center: center,
            dimensions: dimensions,
            orientation: Quaternion::zero(),
            scale: 1.0,
            color: color,
            priority: 0,
            vao: vao,
            vbo: vbo,
            ebo: ebo,
            program: program
        }
    }

    /// Set rendering priority.
    pub fn set_priority(&mut self, priority: u32) {
        self.priority = priority;
    }

    /// Get mutable reference to the cuboid's center.
    pub fn center(&mut self) -> &mut Point3<f32> {
        return &mut self.center;
    }
}

impl Entity for Cuboid {
    fn position(&self) -> Point3<f32> {
        return self.center;
    }

    fn orientation(&self) -> Quaternion<f32> {
        return self.orientation;
    }

    fn scale(&self) -> f32 {
        return self.scale;
    }

    fn move_to(&mut self, position: Point3<f32>) {
        self.center = position;
    }

    fn look_at(&mut self, dir: Vector3<f32>, up: Vector3<f32>) {
        self.orientation = Rotation::look_at(dir, up);
    }

    fn scale_by(&mut self, units: f32) {
        self.scale *= units;
    }

    fn scale_to(&mut self, units: f32) {
        self.scale = units;
    }
}

impl Renderable for Cuboid {
    fn priority(&self) -> u32 {
        return self.priority;
    }

    fn model_matrix(&self) -> Matrix4<f32> {
        let scale_matrix = Matrix4::from_nonuniform_scale(
            self.dimensions.x * self.scale,
            self.dimensions.y * self.scale,
            self.dimensions.z * self.scale);
        let rotation_matrix = Matrix4::from_quat(&self.orientation);
        let translate_matrix = Matrix4::from_translation(self.center.to_vec());

        return translate_matrix * rotation_matrix * scale_matrix;
    }

    fn draw(&self, draw_space: Matrix4<f32>, camera: &Camera) {
        self.vao.bind();
        self.program.bind();

        let mvp_matrix = camera.vp_matrix() * draw_space * self.model_matrix();

        unsafe {
            self.program.uniform("cuboid_color").value(UniformData::FloatVec(4,
                &mem::transmute::<Vector4<f32>, [f32; 4]>(self.color)));

            self.program.uniform("mvp").value(UniformData::FloatMat(4, false,
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

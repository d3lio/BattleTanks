//! This module acts as a playground and thus will be removed in the future.

extern crate engine;
extern crate cgmath;
extern crate gl;

use engine::gliw::{
    Buffer, BufferType, BufferUsagePattern,
    Program,
    Texture, TextureBuilder2D, ImageType, TextureCoordWrap, TextureFilter,
    Uniform, UniformData,
    Vao,
    VertexAttrib, AttribFloatFormat,
};

use engine::core::{Camera, Renderable};

use cgmath::{Vector3, Matrix4};

use std::rc::Rc;
use std::ptr;
use std::mem;

#[allow(dead_code)]
pub struct SimplePlain {
    vao: Vao,
    vbos: Vec<Buffer>,
    program: Rc<Program>,
    model_matrix: Matrix4<f32>,
    attribs: Vec<VertexAttrib>,
    tex: Texture,
}

impl SimplePlain {
    pub fn new(program: Rc<Program>) -> SimplePlain {
        let vao = Vao::new();
        let mut vbos = Vec::<Buffer>::new();

        vao.bind();
        vbos.push(
            Buffer::from_data(
                &VERTEX_DATA,
                BufferType::Array,
                BufferUsagePattern::StaticDraw));
        vbos.push(
            Buffer::from_data(
                &COLOR_DATA,
                BufferType::Array,
                BufferUsagePattern::StaticDraw));

        let model_matrix = Matrix4::from_translation(
            Vector3::<f32>::new(0.0, 0.0, 0.0));

        let mut attribs = Vec::<VertexAttrib>::new();
        attribs.push(VertexAttrib::new(0));
        attribs[0].data_float_format(&vao, &vbos[0], AttribFloatFormat::Float(3), 0, ptr::null());
        attribs.push(VertexAttrib::new(1));
        attribs[1].data_float_format(&vao, &vbos[1], AttribFloatFormat::Float(2), 0, ptr::null());

        let tex = TextureBuilder2D::new()
            .source("resources/textures/banana.bmp", ImageType::Bmp)
            .wrap(TextureCoordWrap::Repeat, TextureCoordWrap::Repeat)
            .filter(TextureFilter::LinearMipmapLinear, TextureFilter::Linear)
            .gen_mipmap()
            .load()
            .unwrap();

        tex.pass_to(&program, "tex", 0);

        return SimplePlain {
            vao: vao,
            vbos: vbos,
            program: program,
            model_matrix: model_matrix,
            attribs: attribs,
            tex: tex
        };
    }
}

impl Renderable for SimplePlain {
    fn model_matrix(&self) -> Matrix4<f32> {
        return self.model_matrix;
    }

    fn draw(&self, draw_space: Matrix4<f32>, camera: &Camera) {
        self.vao.bind();
        self.program.bind();

        let mvp_matrix = camera.vp_matrix() * draw_space * self.model_matrix;

        unsafe {
            Uniform::new(&self.program, "mvp").value(UniformData::FloatMat(4, false,
                &mem::transmute::<Matrix4<f32>, [f32; 16]>(mvp_matrix)));
        }

        for attrib in &self.attribs {
            attrib.enable(&self.vao);
        }

        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 6); }

        for attrib in &self.attribs {
            attrib.disable(&self.vao);
        }
    }
}

static VERTEX_DATA: [f32; 18] = [
    -0.5, 1.0, 0.501,
    -0.5, 0.0, 0.501,
     0.5, 0.0, 0.501,

    -0.5, 1.0, 0.501,
     0.5, 0.0, 0.501,
     0.5, 1.0, 0.501,
];

static COLOR_DATA: [f32; 12] = [
    4.0, 4.0,
    4.0, 0.0,
    0.0, 0.0,

    4.0, 4.0,
    0.0, 0.0,
    0.0, 4.0,
];

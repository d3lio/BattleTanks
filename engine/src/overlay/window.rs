extern crate cgmath;

use self::cgmath::{Vector2, Vector3, Vector4, Vector};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt::{self, Debug, Formatter};

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;

// TODO: replace Option<Weak<_>> with Weak<_> when `downgraded_weak` is stabilized
// #[derive(Debug)]
pub struct Window {
    pub name: String,
    pub creation_data: WindowParams,

    pub pos: Vec2,
    pub size: Vec2,
    pub shown: bool,    // TODO: move inside WindowParams?

    pub children: Vec<Rc<Box<RefCell<Window>>>>,
    pub parent: Option<Weak<Box<RefCell<Window>>>>,

    pub vbo_beg: isize,
    pub vbo_end: isize,
}

impl Window {
    pub fn new(name: &str, data: WindowParams) -> Window {
        return Window {
            name: String::from(name),
            creation_data: data,
            pos: Vec2::zero(),
            size: Vec2::zero(),
            shown: true,
            children: Vec::new(),
            parent: None,
            vbo_beg: -1,
            vbo_end: -1,
        };
    }
}

#[derive(Clone, Copy)]
pub struct WindowParams {
    pub pos: Vector2<Vec3>,
    pub size: Vector2<Vec3>,
    pub color: [Vec4; 4],
    pub texcoord: [Vec2; 4],
}

impl Debug for WindowParams {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "WindowParams{{ ... }}")
    }
}


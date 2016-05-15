//! A 2D Overlay
//!
//! An overlay object represents something that is drawn on a portion of the screen after the rest of the
//! scene has been rendered.
//!
//! It uses its own relative coordinate system, where the center (0, 0) is the leftmost uppermost point,
//! the positive X axis points right, the positive Y axis points down and one unit is one pixel.
//!
//! An `Overlay` is composed of several `Window` objects, attached together in a hierarchal structure.
//! Each overlay has a root window which encompasses the whole overlay area (which usually is the whole vieport).
//! Only windows attached to the root are rendered.
//!
//! Each window has a name, which is a string not containing the `.` character. Several window names, separated
//! by a `.` character represents a path in the hierarchy three. Paths are used for identification, so while it
//! is possible to have two windows with the same name, two different paths cannot have the same string representation.
//!
//! Each `Window` represents a rectangular area of the screen that is drawn over.
//! Windows are represented by the four vertices of the rectangle:
//!
//!   * vertex 0 - upper left
//!   * vertex 1 - upper right
//!   * vertex 2 - bottom left
//!   * vertex 3 - bottom right
//!
//! The order of rendering is dependent on the order in which windows are attached to each other. First is
//! rendered the subtree of the first attached child, then the subtree of the next attached child, etc,
//! continuing recursively using the same algorithm.
//! In other words it is a pre-order traversal of the hierarchy tree.

mod overlay;
mod window;

extern crate cgmath;

use gliw::{Program, Vao, Vbo};

use self::cgmath::{Vector2, Vector3, Vector4};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;

struct OverlayData {
    vao: Vao,
    vbo: Vbo,
    prog: Program,
    indices: Vec<u32>,      // TODO: maybe make the indices Vec<u16>

    root: Window,

    should_reindex: bool,
}

pub struct Overlay(RefCell<OverlayData>);

#[derive(Debug)]
struct WindowData {
    name: String,
    creation_data: WindowParams,
    overlay: *mut OverlayData,

    pos: Vec2,
    size: Vec2,

    children: Vec<Window>,
    parent: WindowWeak,

    vbo_beg: isize,
    vbo_end: isize,
}

#[derive(Debug, Clone)]
pub struct Window(Rc<Box<RefCell<WindowData>>>);

// TODO: replace Option<Weak<_>> with Weak<_> when `downgraded_weak` is stabilized
#[derive(Debug, Clone)]
struct WindowWeak(Option<Weak<Box<RefCell<WindowData>>>>);

#[derive(Clone, Copy)]
pub struct WindowParams {
    pub pos: Vector2<Vec3>,
    pub size: Vector2<Vec3>,
    pub color: [Vec4; 4],
    pub texcoord: [Vec2; 4],
    pub shown: bool,
}


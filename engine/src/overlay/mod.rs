//! A 2D Overlay
//!
//! An `Overlay` object represents a collection of 2D items that are rendered on the screen after the rest
//! of the scene.
//!
//! It uses its own relative coordinate system, where the center (0, 0) is the leftmost uppermost point,
//! the positive X axis points right, the positive Y axis points down and one unit is one pixel.
//!
//! An `Overlay` is composed of a tree of `Window` objects. The overlay contains the tree root (the root `Window`).
//! All windows which should be rendered must be attached to the root.
//!
//! A `Window` is a single item on the overlay. More technically a window is a rectangular area that is rendered
//! filled with either a color or a texture. Windows can contain child windows which inherit their position and size
//! (see struct `WindowParams` for more info).
//!
//! The four vertices of the rectangle are enumereated in the following order:
//!
//!   * vertex 0 - upper left
//!   * vertex 1 - upper right
//!   * vertex 2 - bottom left
//!   * vertex 3 - bottom right
//!
//! Each window has a name, which is a string not containing the `/` character. Names can be concatinated, separated
//! by the `/` character to create paths, which can be used to identify windows in the hierarchy tree.
//! Since the paths are used as identifiers they must be unique within the same overlay.
//! However the is no such restriction for window names - there can be multiple windows with the same name.
//!
//! The order of rendering is dependent on the order in which windows are attached to each other -
//! it is a pre-order traversal of the hierarchy tree.

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

/// 2D overlay
///
/// See the module level documentation for more info.
pub struct Overlay(Box<OverlayData>);

#[derive(Debug)]
struct WindowData {
    name: String,
    params: WindowParams,
    overlay: *mut OverlayData,

    pos: Vec2,
    size: Vec2,

    children: Vec<Window>,
    parent: WindowWeak,

    index_beg: usize,
    index_end: usize,
}

/// A single item on the overlay
///
/// See the module level documentation for more info.
///
/// This class is a reference counted wrapper to the internal window data, so
/// cloning a `Window` creates another reference to the same internal data.
///
#[derive(Debug, Clone)]
pub struct Window(Rc<Box<RefCell<WindowData>>>);

// TODO: replace Option<Weak<_>> with Weak<_> when `downgraded_weak` is stabilized
#[derive(Debug, Clone)]
struct WindowWeak(Option<Weak<Box<RefCell<WindowData>>>>);

/// The parameters of a window, which determine how it should be rendered
///
/// These parameters are set once a window is created and can later be changed
/// using the `Window::modify` method.
///
/// The positional parameters (`pos` and `size`) determine position relative to the parent
/// window. In case that is the root window it has the following coordinates.
///
/// ```ignore
/// root.pos_x = 0;
/// root.pos_y = 0;
/// root.width = the width of the overlay area;
/// root.height = the height of the overlay area;
/// ```
///
#[derive(Clone, Copy)]
pub struct WindowParams {
    /// The `XY` coordinates of the upper left corner relative to the parent window.
    ///
    /// ```ignore
    /// pos: vec2(vec3(px1, px2, px3), vec3(py1, py2, py3))
    /// ```
    /// results in the equation
    ///
    /// ```ignore
    /// window.pos_x = parent.pos_x + (px1 * parent.width + px2 * parent.height + px3)
    /// window.pos_y = parent.pos_y + (py1 * parent.width + py2 * parent.height + py3)
    /// ```
    ///
    /// Notice that `px1`, `px2`, `py1` and `py2` are ratios
    /// while `px3` and `py3` are in pixels.
    pub pos: Vector2<Vec3>,

    /// The `width` and `height` of the window relative to the parent window
    ///
    /// ```ignore
    /// size: vec2(vec3(px1, px2, px3), vec3(py1, py2, py3))
    /// ```
    /// results in the equation
    ///
    /// ```ignore
    /// window.width  = px1 * parent.width + px2 * parent.height + px3
    /// window.height = py1 * parent.width + py2 * parent.height + py3
    /// ```
    ///
    /// Again `px1`, `px2`, `py1` and `py2` are ratios
    /// while `px3` and `py3` are in pixels.
    pub size: Vector2<Vec3>,

    /// The colors at the four vertices of the rectangle.
    /// The format is `vec4(r, g, b, a)` with values between `0.0` and `1.0`.
    pub color: [Vec4; 4],

    /// Should be the `UV` coordinates of the texture but is currently unused :(
    pub texcoord: [Vec2; 4],

    /// Controls whether the window is visible or not.
    ///
    /// If `shown` is `false` the window and all of its children are hidden. They are
    /// still rendered, but outside the viewport.
    ///
    /// Setting `shown` to `false` is good if you want to hide the window for a few frames.
    /// If you want to permanently hide it you should consider using `Window::detach` instead.
    pub shown: bool,
}

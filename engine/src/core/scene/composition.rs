extern crate cgmath;

use self::cgmath::Matrix4;

use super::node_container::NodeContainer;

use super::camera::Camera;
use super::renderable::Renderable;

use std::rc::Weak;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

/// A structure used for holding relative objects.
///
/// This structure provides a way to render relative objects.
/// It's a wrapper for any `Renderable` and has a `NodeContainer` for child objects.
/// What this means is you can use the composition just like you would use the object it wraps
/// thanks to the derefs, with the minor difference that it has an `add` function
/// to hook objects together.
///
/// Since the `Composition` is a `Renderable` itself we come across its most powerful
/// feature - it can hold other `Composition`s as children.
pub struct Composition<T: Renderable> {
    renderable: T,
    children: RefCell<NodeContainer>
}

impl<T: Renderable> Composition<T> {
    /// Create a new `Composition` wrapper for a `Renderable`.
    pub fn new(renderable: T) -> Composition<T> {
        return Composition {
            renderable: renderable,
            children: RefCell::new(NodeContainer::new())
        };
    }

    /// Adds a relative `Renderable` object.
    ///
    /// See `Scene.add` for more info.
    pub fn add<R>(&mut self, renderable: Weak<RefCell<R>>) -> &mut Self
        where R: Renderable + 'static
    {
        self.children.borrow_mut().add(renderable);

        return self;
    }
}

impl<T: Renderable> Renderable for Composition<T> {
    fn priority(&self) -> u32 {
        return self.renderable.priority();
    }

    fn model_matrix(&self) -> Matrix4<f32> {
        return self.renderable.model_matrix();
    }

    fn draw(&self, draw_space: Matrix4<f32>, camera: &Camera) {
        self.renderable.draw(draw_space, camera);

        self.children.borrow_mut().retain(|child_wk| {
            match child_wk.upgrade() {
                Some(child) => {
                    child.borrow().draw(draw_space * self.renderable.model_matrix(), camera);
                    return true;
                },
                None => return false
            }
        });
    }
}

impl<T: Renderable> Deref for Composition<T> {
    type Target = T;

    fn deref(&self) -> &T {
        return &self.renderable;
    }
}

impl<T: Renderable> DerefMut for Composition<T> {
    fn deref_mut(&mut self) -> &mut T {
        return &mut self.renderable;
    }
}

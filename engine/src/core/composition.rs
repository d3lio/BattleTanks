extern crate cgmath;

use self::cgmath::Matrix4;

use core::{Camera, Renderable};

use std::rc::Weak;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

/// A structure used for holding relative objects.
///
/// This structure provides a way to render relative objects.
/// It's a wrapper for any `Renderable` and has a vector of `Rederable` child objects.
/// What this means is you can use the composition just like you would use the object it wraps
/// thanks to the derefs, with the minor difference that it has an `add` function
/// to hook objects together.
///
/// Since the `Composition` is a `Renderable` itself we come across its most powerful
/// feature - it can hold other `Composition`s as children.
pub struct Composition<T: Renderable + 'static> {
    children: RefCell<Vec<Weak<RefCell<Renderable>>>>,
    renderable: T
}

impl<T: Renderable> Composition<T> {
    /// Create a new `Composition` wrapper for a `Renderable`.
    pub fn new(renderable: T) -> Composition<T> {
        return Composition {
            children: RefCell::new(Vec::new()),
            renderable: renderable
        };
    }

    /// Adds a relative `Renderable` object.
    ///
    /// See `Scene.add` for more info.
    pub fn add<R>(&mut self, renderable: Weak<RefCell<R>>) -> &mut Self
        where R: Renderable + 'static
    {
        let ins_priority = match renderable.upgrade() {
            Some(renderable_ref) => renderable_ref.borrow().priority(),
            None => return self
        };

        let mut ins_pos: usize = 0;
        let mut found: bool = false;

        self.children.borrow_mut().retain(|child_ref| {
            match child_ref.upgrade() {
                Some(child) => {
                    if !found {
                        // < is preffered than <= for better performance.
                        // This affects priority, see `Scene::add`.
                        if ins_priority < child.borrow().priority() {
                            found = true;
                        } else {
                            ins_pos += 1;
                        }
                    }
                    return true;
                },
                None => return false
            }
        });

        self.children.borrow_mut().insert(ins_pos, renderable);

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

        self.children.borrow_mut().retain(|child_ref| {
            match child_ref.upgrade() {
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

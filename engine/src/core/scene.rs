use core::renderable::Renderable;

use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// A scene structure holding `Renderable` objects.
///
/// It sustains itself by erasing any invalid `Weak` refs in the rendering queue.
pub struct Scene {
    render_queue: RefCell<Vec<Weak<Renderable>>>
}

impl Scene {
    /// Create a new `Scene`.
    pub fn new() -> Scene {
        return Scene {
            render_queue: RefCell::new(Vec::new())
        };
    }

    /// Add a `Renderable` object to the scene.
    ///
    /// Make sure you are cloning your Rc otherwise the scene will optimize it out.
    pub fn add(&mut self, ent_ref: Rc<Renderable>) -> &mut Self {
        // The &mut self can be just &self but this way
        // it shows the logical mutation.
        self.render_queue.borrow_mut().push(Rc::downgrade(&ent_ref));
        return self;
    }

    /// Draw all `Renderable` objects.
    pub fn draw(&self) {
        self.render_queue.borrow_mut().retain(|ent_ref| {
            match ent_ref.upgrade() {
                Some(ent) => {
                    ent.draw();
                    return true;
                },
                None => return false
            }
        });
    }
}

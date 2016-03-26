use core::renderable::Renderable;

use std::rc::Weak;
use std::cell::RefCell;

/// A scene structure holding `Renderable` objects.
///
/// It sustains itself by removing any invalid `Weak` refs in the rendering queue.
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
    /// `ent_ref` is ignored if it's data was destroyed and has no remaining strong refs.
    pub fn add<R>(&mut self, ent_ref: Weak<R>) -> &mut Self
        where R: Renderable + 'static
    {
        let ent_priority = match ent_ref.upgrade() {
            Some(ent) => ent.priority(),
            None => return self
        };

        let mut ent_pos: usize = 0;
        let mut found: bool = false;

        self.render_queue.borrow_mut().retain(|ent_ref| {
            match ent_ref.upgrade() {
                Some(ent) => {
                    if !found {
                        if ent.priority() >= ent_priority {
                            found = true;
                        } else {
                            ent_pos += 1;
                        }
                    }
                    return true;
                },
                None => return false
            }
        });

        // The &mut self can be just &self but this way
        // it shows the logical mutation.
        self.render_queue.borrow_mut().insert(ent_pos, ent_ref);

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

use core::{Camera, Renderable};

use std::rc::Weak;
use std::cell::RefCell;

/// A scene structure holding `Renderable` objects.
///
/// The scene uses a render priority system where the lower priority targets will be rendered earlier
/// meaning that they will get overlapped by higher priority objects.
/// It also sustains itself by removing any invalid `Weak` refs from the rendering queue.
pub struct Scene {
    camera: Camera,
    render_queue: RefCell<Vec<Weak<RefCell<Renderable>>>>
}

impl Scene {
    /// Create a new `Scene`.
    pub fn new(camera: Camera) -> Scene {
        return Scene {
            camera: camera,
            render_queue: RefCell::new(Vec::new())
        };
    }

    /// Get mutable reference to the scene's camera.
    pub fn camera_mut(&mut self) -> &mut Camera {
        return &mut self.camera;
    }

    /// Add a `Renderable` object to the scene.
    ///
    /// When adding two or more renderables with the same priority,
    /// the earlier added will have lower priority.
    pub fn add<R>(&mut self, renderable: Weak<RefCell<R>>) -> &mut Self
        where R: Renderable + 'static
    {
        // The &mut self can be just &self but this way it shows the logical mutation.

        let ent_priority = match renderable.upgrade() {
            Some(ent) => ent.borrow().priority(),
            None => return self
        };

        let mut ent_pos: usize = 0;
        let mut found: bool = false;

        self.render_queue.borrow_mut().retain(|ent_ref| {
            match ent_ref.upgrade() {
                Some(ent) => {
                    if !found {
                        // < is preffered than <= for better performance.
                        // This affects priority, see `Scene::add`.
                        if ent_priority < ent.borrow().priority() {
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

        self.render_queue.borrow_mut().insert(ent_pos, renderable);

        return self;
    }

    /// Draw all `Renderable` objects.
    pub fn draw(&self) {
        self.render_queue.borrow_mut().retain(|ent_ref| {
            match ent_ref.upgrade() {
                Some(ent) => {
                    ent.borrow().draw(&self.camera);
                    return true;
                },
                None => return false
            }
        });
    }
}

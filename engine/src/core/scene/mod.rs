extern crate cgmath;

mod node_container;

pub mod camera;
pub mod composition;
pub mod renderable;

use self::cgmath::{Matrix4, SquareMatrix};

use self::node_container::NodeContainer;

use self::camera::Camera;
use self::renderable::Renderable;

use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// A structure used for rendering `Renderable` objects.
///
/// The scene uses a render priority system where the lower priority targets will be rendered earlier
/// meaning that they will get overlapped by higher priority objects.
/// It also sustains itself by removing any invalid `Weak` refs from the rendering queue.
pub struct Scene {
    camera: Camera,
    render_queue: RefCell<NodeContainer>
}

impl Scene {
    /// Create a new `Scene`.
    pub fn new(camera: Camera) -> Scene {
        return Scene {
            camera: camera,
            render_queue: RefCell::new(NodeContainer::new())
        };
    }

    /// Get mutable reference to the scene's camera.
    pub fn camera_mut(&mut self) -> &mut Camera {
        return &mut self.camera;
    }

    /// Downgrade a wrapped `Renderable`.
    ///
    /// See `engine::wrap!`
    #[inline]
    pub fn node<R: Renderable>(renderable: &Rc<RefCell<R>>) -> Weak<RefCell<R>> {
        NodeContainer::node(renderable)
    }

    /// Add a `Renderable` object to the scene.
    ///
    /// When adding two or more renderables with the same priority,
    /// the earlier added will have lower priority.
    pub fn add<R>(&mut self, renderable: Weak<RefCell<R>>) -> &mut Self
        where R: Renderable + 'static
    {
        // The &mut self can be just &self but this way it shows the logical mutation.

        self.render_queue.borrow_mut().add(renderable);

        return self;
    }

    /// Draw all `Renderable` objects.
    pub fn draw(&self) {
        self.render_queue.borrow_mut().retain(|renderable_wk| {
            match renderable_wk.upgrade() {
                Some(renderable) => {
                    renderable.borrow().draw(Matrix4::identity(), &self.camera);
                    return true;
                },
                None => return false
            }
        });
    }
}

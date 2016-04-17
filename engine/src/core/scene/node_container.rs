//! Internal container.

use core::Renderable;

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

// FIXME: if the priority of an object changes prior to it's insertion,
// it will break the priority mechanism.

/// A container for wrapped `Renderables`.
///
/// See `Scene` for more info on how this object works.
/// See `NodeContainer.wrap`.
pub struct NodeContainer {
    container: Vec<Weak<RefCell<Renderable>>>
}

impl NodeContainer {
    /// Create a new `NodeContainer`.
    pub fn new() -> NodeContainer {
        return NodeContainer {
            container: Vec::new()
        };
    }

    /// See `Scene.wrap`.
    #[inline]
    pub fn wrap<R: Renderable>(renderable: R) -> Rc<RefCell<R>> {
        return Rc::new(RefCell::new(renderable));
    }

    /// See `Scene.node`.
    #[inline]
    pub fn node<R: Renderable>(renderable: &Rc<RefCell<R>>) -> Weak<RefCell<R>> {
        return Rc::downgrade(renderable);
    }

    /// Add a wrapped `Renderable`.
    ///
    /// Since `Vec` does not have an `add` function it is safe to use `self`
    /// instead of `this` with the `Deref` trait.
    ///
    /// See `Scene.add`.
    pub fn add<R>(&mut self, node: Weak<RefCell<R>>)
        where R: Renderable + 'static
    {
        // The &mut self can be just &self but this way it shows the logical mutation.

        let node_priority = match node.upgrade() {
            Some(node_rc) => node_rc.borrow().priority(),
            None => return
        };

        let mut ins_pos: usize = 0;
        let mut found: bool = false;

        self.container.retain(|node_wk| {
            match node_wk.upgrade() {
                Some(node_rc) => {
                    if !found {
                        // < is preffered than <= for better performance.
                        // This way less elements will be moved with the insertion.
                        // This affects priority, see `Scene::add`.
                        if node_priority < node_rc.borrow().priority() {
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

        self.container.insert(ins_pos, node);
    }
}

impl Deref for NodeContainer {
    type Target = Vec<Weak<RefCell<Renderable>>>;

    fn deref(&self) -> &Self::Target {
        return &self.container;
    }
}

impl DerefMut for NodeContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.container;
    }
}

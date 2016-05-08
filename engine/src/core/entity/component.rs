use super::Entity;

use std::cell::RefCell;
use std::rc::Weak;

/// Marks a structure to be a component used by entities.
pub trait Component {
    /// This method is called by the `add` method of an entity.
    ///
    /// It injects some useful initialisation data to allow the user to attach listeners.
    fn subscribe(&mut self, weak: Weak<RefCell<Self>>, entity: &mut Entity);
}

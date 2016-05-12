use super::Entity;

use core::{Data, Event};

use std::any::Any;

/// Marks a structure to be a component used by entities.
pub trait Component {
    /// This method is called by the `add` method of an entity.
    ///
    /// It injects some useful initialisation data to allow the user to attach listeners.
    fn subscribe(&mut self, entity: &mut Entity) -> Vec<(Vec<Event>, Box<Fn(&Any, &Event, &Data)>)>;
}

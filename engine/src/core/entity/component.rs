use super::Entity;

use core::{Data, Event};

use std::any::Any;

/// Listener subscription callback type used by `Component`s
pub type SubCallback = Fn(Vec<Event>, Box<Fn(&Any, &Event, &Data)>);

/// Marks a structure to be a component used by entities.
pub trait Component {
    /// This method is called by the `add` method of an entity.
    ///
    /// It injects some useful initialisation data to allow the user to attach listeners.
    fn init(&mut self, entity: &mut Entity, on: &SubCallback);
}

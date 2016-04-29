use super::PropContainer;

use core::EventEmitter;

use std::any::Any;
use std::cell::RefCell;
use std::rc::Weak;

// FIXME: should not have self and weak together
// TODO: inject a timer somehow
// TODO: component! macro

/// Marks a structure to be a component used by entities.
pub trait Component {
    /// This method is called by the `add` method of an entity.
    ///
    /// It injects some useful initialisation data to allow the user to attach listeners.
    fn subscribe(&mut self, weak: Weak<RefCell<Self>>, props: &PropContainer,
        emitter: &mut EventEmitter<Any, PropContainer>);
}

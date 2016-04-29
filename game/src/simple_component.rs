//! This module acts as a playground and thus will be removed in the future.

extern crate engine;

use engine::core::{PropContainer, Component, Event, Listener, EventEmitter};

use std::any::Any;
use std::cell::RefCell;
use std::rc::Weak;

pub struct SimpleComponent {
    wow: &'static str
}

impl SimpleComponent {
    pub fn new() -> SimpleComponent {
        return SimpleComponent {
            wow: "wow"
        }
    }
}

impl Component for SimpleComponent {
    fn subscribe(&mut self, weak: Weak<RefCell<Self>>, _: &PropContainer,
        emitter: &mut EventEmitter<Any, PropContainer>)
    {
        self.wow = "wow!";
        emitter.on(Event("update"), Listener::<Any, PropContainer>::new(weak.clone(),
            |this, _, _, _| {
                let component = this.downcast_ref::<RefCell<Self>>().unwrap();
                assert_eq!(component.borrow().wow, "wow!");
            }
        ));
    }
}

//! Input managment system

extern crate glfw;

#[macro_use]
mod key_range;
mod manager;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use self::manager::InputManagerData;

pub use self::key_range::KeyRange;
pub use self::manager::InputManager;

#[derive(Clone)]
pub struct Listener {
    closure: Rc<Box<Fn(glfw::Key, glfw::Scancode, glfw::Action)>>,
    capture: bool,
}

impl Listener {
    pub fn new<F> (callback: F, capture: bool) -> Listener where
        F: Fn(glfw::Key, glfw::Scancode, glfw::Action) + 'static
    {
        Listener {
            closure: Rc::new(Box::new(callback)),
            capture: capture
        }
    }

    #[inline(always)]
    fn call(&self, key: glfw::Key, scancode: glfw::Scancode, action: glfw::Action) {
        (*self.closure)(key, scancode, action);
    }
}

pub struct KeyFocus {
    listeners: Vec<(KeyRange, Listener)>,
    manager: Option<Weak<RefCell<InputManagerData>>>,
}

impl KeyFocus {
    pub fn new() -> KeyFocus {
        KeyFocus {
            listeners: Vec::new(),
            manager: None,
        }
    }

    pub fn on(&mut self, keys: KeyRange, listener: Listener) {
        self.listeners.push((keys, listener));
    }

    pub fn key_pressed(&self, key: glfw::Key) -> bool {
        match self.manager() {
            Some(mgr) => mgr.borrow().key_unbuffered(self, key),
            None => false,
        }
    }

    // FIXME: temporary until `downgraded_weak` is stabilized
    fn manager(&self) -> Option<Rc<RefCell<InputManagerData>>> {
        match self.manager {
            Some(ref weak) => weak.upgrade(),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;

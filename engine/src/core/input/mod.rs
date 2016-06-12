//! Input managment system

// A system to manage the distribution of input events. Consists of a single manager class and multiple listeners.
// The manager is fed all input events from the system. It also keeps track of the listeners using a stack-like structure. Whenever a listener receives focus it is moved to the top of the stack.
// Events are send to the listeners in a capturing manner, starting at the top of the stack. I.e. only one listener can receive an event and that is the listener closest to the top of the stack that has the corresponding callback set (unless passtrough ...).

extern crate glfw;

#[macro_use]
mod mask;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub use self::mask::KeyMask;

pub struct KeyListener {
    subscribers: Vec<(KeyMask, Rc<Fn(glfw::Key, glfw::Scancode, glfw::Action)>, bool)>,
    manager: ManagerWeak,
}

struct _Manager {
    key_listeners: Vec<*mut KeyListener>,
    window: *const glfw::Window,
}

pub struct Manager (Rc<RefCell<_Manager>>);
struct ManagerWeak (Option<Weak<RefCell<_Manager>>>);


impl KeyListener {
    pub fn new() -> KeyListener {
        KeyListener {
            subscribers: Vec::new(),
            manager: ManagerWeak(None),
        }
    }

    pub fn on<F> (&mut self, keys: KeyMask, callback: Rc<F>, passtrough: bool) where
        F: Fn(glfw::Key, glfw::Scancode, glfw::Action) + 'static
    {
        self.subscribers.push((keys, callback, passtrough));
    }

    pub fn key_pressed(&self, key: glfw::Key) -> bool {
        match self.manager.upgrade() {
            Some(mgr) => mgr.key_unbuffered(self, key),
            None => false,
        }
    }
}

impl Drop for KeyListener {
    fn drop(&mut self) {
        if let Some(mgr) = self.manager.upgrade() {
            mgr.lose_key_focus(self);
        }
    }
}

impl _Manager {
    // FIXME: no lifetime checks
    fn new (window: &glfw::Window) -> _Manager {
        _Manager {
            key_listeners: Vec::new(),
            window: window as *const _,
        }
    }
}

impl Manager {
    pub fn new(window: &glfw::Window) -> Manager {
        Manager(wrap!(_Manager::new(window)))
    }

    pub fn gain_key_focus(&self, focus: &mut KeyListener) {
        if let Some(mgr) = focus.manager.upgrade() {
            mgr.lose_key_focus(focus);
        }

        self.0.borrow_mut().key_listeners.push(focus as *mut _);
        focus.manager = ManagerWeak(Some(Rc::downgrade(&self.0)));
    }

    pub fn lose_key_focus(&self, focus: &mut KeyListener) {
        let focus_ptr = focus as *mut _;
        self.0.borrow_mut().key_listeners.retain(|&lptr| lptr != focus_ptr);
        focus.manager = ManagerWeak(None);
    }

    pub fn key_unbuffered(&self, listener: &KeyListener, key: glfw::Key) -> bool {
        unsafe {
            let mgr = self.0.borrow();

            for &lptr in mgr.key_listeners.iter().rev() {
                if lptr as *const _ == listener as *const _ {
                    for &(ref mask, _, _) in &(*lptr).subscribers {
                        if mask.check(key) {
                            assert!(!mgr.window.is_null());
                            return (*mgr.window).get_key(key) == glfw::Action::Press;
                        }
                    }

                    return false;
                }

                for &(ref mask, _, passtrough) in &(*lptr).subscribers {
                    if mask.check(key) && !passtrough {
                      return false;
                    }
                }
            }
            return false;
        }
    }

    pub fn emit_key(&self, key: glfw::Key, scancode: glfw::Scancode, action: glfw::Action) {
        unsafe {
            for &listener in self.0.borrow().key_listeners.iter().rev() {
                for &(ref mask, ref callback, passtrough) in &(*listener).subscribers {
                    if mask.check(key) {
                        callback(key, scancode, action);
                        if !passtrough {
                            return;
                        }
                    }
                }
            }
        }
    }
}

impl ManagerWeak {
    fn upgrade(&self) -> Option<Manager> {
        match self.0 {
            Some(ref weak) => match weak.upgrade() {
                Some(mgr_rc) => Some(Manager(mgr_rc)),
                None => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests;

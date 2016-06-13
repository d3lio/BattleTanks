//! Input management system
//!
//! A system to manage the distribution of input events.
//! Consists of a manager class and multiple listener classes for different events.
//!
//! The listener classes represent a single object that can receive input events.
//! Callback functions can be subscribed for the various events.
//! Listeners have the concept of "focus". When the user "selects" the object the listener
//! is said to receive input focus. When the user "deselects" it the listener loses input
//! focus. The listener can only receive events when it has focus.
//!
//! The manager is fed all input events from the system to distribute them among the listeners.
//! It also keeps s stack of all listeners who are currently under focus, with the ones who most
//! recently received focus being on the top of the stack.
//!
//! Events are send to the listeners in a capturing manner, starting at the top of the stack.
//! The event is received by the listener closest to the top of the stack who has the corresponding
//! callback function set. If that listener has the `passtrough` option set then that event is also
//! passed to the listeners bellow him.

extern crate glfw;

#[macro_use]
mod mask;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub use self::mask::KeyMask;

// TODO: keep track of all key pressed events received and fire the
// corresponding key released events upon losing focus.

/// Listener for keyboard input events.
///
/// Listens for individual key strokes as determined by their keycodes and is thus independent
/// of the current keyboard layout the user has selected.
pub struct KeyListener {
    subscribers: Vec<(KeyMask, Rc<Fn(glfw::Key, glfw::Scancode, glfw::Action)>, bool)>,
    manager: ManagerWeak,
}

struct _Manager {
    key_listeners: Vec<*mut KeyListener>,
    window: *const glfw::Window,
}

/// Input event manager.
///
/// The manager should be fed all input events from the system event queue using one
/// of the `emit_*` methods and it will distribute those events to all listeners who
/// are currently on focus.
///
/// Usually only a single instance of this class per game window should be created.
pub struct Manager (Rc<RefCell<_Manager>>);

struct ManagerWeak (Option<Weak<RefCell<_Manager>>>);

impl KeyListener {
    /// Create a new `KeyListener`.
    pub fn new() -> KeyListener {
        KeyListener {
            subscribers: Vec::new(),
            manager: ManagerWeak(None),
        }
    }

    /// Set a callback for a set of keys.
    ///
    /// `keys` is the set of keys for who the callback will be triggered.
    ///
    /// `callback` is the callback function. It receives the keycode, system specific scan code
    /// and the action - pressed, released or repeat.
    ///
    /// `passtrough` should be set to `true` if you want other listeners to receive the same event.
    ///
    pub fn on<F> (&mut self, keys: KeyMask, callback: Rc<F>, passtrough: bool) where
        F: Fn(glfw::Key, glfw::Scancode, glfw::Action) + 'static
    {
        self.subscribers.push((keys, callback, passtrough));
    }

    /// Get the state of the key since the last polling of input events.
    ///
    /// This function follows the same rules as the event callbacks, i.e. listener
    /// has to have focus and a callback function for `key` must have been set.
    ///
    /// The state of `key` will be returned only if a callback function for `key` from
    /// this listener would have been called. In that case `true` is returned if the key
    /// is pressed and `false` otherwise.
    ///
    /// In all other cases `false` is returned.
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
    /// Create a new manager.
    ///
    /// `window` is a handle to the window object that will be used for polling buffered input.
    pub fn new(window: &glfw::Window) -> Manager {
        Manager(wrap!(_Manager::new(window)))
    }

    /// Notify the manager that a listener has gained input focus.
    ///
    /// This puts the listener on the top of the stack.
    pub fn gain_key_focus(&self, focus: &mut KeyListener) {
        if let Some(mgr) = focus.manager.upgrade() {
            mgr.lose_key_focus(focus);
        }

        self.0.borrow_mut().key_listeners.push(focus as *mut _);
        focus.manager = ManagerWeak(Some(Rc::downgrade(&self.0)));
    }

    /// Notify the manager that a listener has gained input focus.
    pub fn lose_key_focus(&self, focus: &mut KeyListener) {
        let focus_ptr = focus as *mut _;
        self.0.borrow_mut().key_listeners.retain(|&lptr| lptr != focus_ptr);
        focus.manager = ManagerWeak(None);
    }

    /// Feed the manager.
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

    fn key_unbuffered(&self, listener: &KeyListener, key: glfw::Key) -> bool {
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

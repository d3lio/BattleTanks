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
///
/// Received events are buffered in order to provide the following guarantees. For any given key: <br>
/// The first event received will always be `Press`. <br>
/// For every `Press` event there will be a matching `Release` event.
///
/// If the corresponding `Release` events have not been received when the listener loses focus, they will
/// be triggered in an arbitrary order.
///
pub struct KeyListener {
    keys: KeyMask,
    passtrough: bool,
    callback: Box<Fn(glfw::Key, glfw::Scancode, glfw::Action)>,

    pressed: KeyMask,
    manager: ManagerWeak,
}

struct _Manager {
    key_listeners: Vec<*mut KeyListener>,
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
    /// Create a new listener.
    ///
    /// The listener will capture events for the specified keys in `keys` and will trigger the callback
    /// function for each event.
    ///
    /// If `passtrough` is `false` event propagation will stop after this listener captures the event.
    /// Set `passtrough` to `true` if you want the event to be propagated to other listeners down the chain.
    ///
    pub fn new<F> (keys: KeyMask, passtrough: bool, callback: F) -> KeyListener where
        F: Fn(glfw::Key, glfw::Scancode, glfw::Action) + 'static
    {
        KeyListener {
            keys: keys,
            passtrough: passtrough,
            callback: Box::new(callback),
            pressed: KeyMask::new(),
            manager: ManagerWeak(None),
        }
    }

    /// Buffered input.
    ///
    /// This method pool the state from the internal event buffer, meaning that only states of the keys
    /// for which the callback function is listening are tracked.
    ///
    pub fn key_pressed(&self, key: glfw::Key) -> bool {
        self.pressed.check(key)
    }

    /// Notify the manager that the listener has gained focus.
    ///
    /// # Panics
    ///
    /// If the listener is currently on focus in a different manager.
    ///
    pub fn gain_focus(&mut self, mgr: &Manager) {
        if let Some(ref prev_mgr) = self.manager.upgrade() {
            if prev_mgr != mgr {
                panic!(ERR_DIFF_MANAGER);
            }
            return;
        }

        mgr.gain_key_focus(self);
        self.manager = ManagerWeak(Some(Rc::downgrade(&mgr.0)));
    }

    /// Notify the manager that the listener has lost focus.
    ///
    /// The manager is stored internally, which is why it is not passed as a parameter.
    /// If the listener was not under focus this method does nothing.
    ///
    pub fn lose_focus(&mut self) {
        if let Some(mgr) = self.manager.upgrade() {
            mgr.lose_key_focus(self);
        }

        for key in &self.pressed {
            (self.callback)(key, 0, glfw::Action::Release);
        }

        self.pressed = KeyMask::new();
        self.manager = ManagerWeak(None);
    }

    fn call(&mut self, key: glfw::Key, scancode: glfw::Scancode, action: glfw::Action) {
        match action {
            glfw::Action::Press => {
                if !self.pressed.check(key) {
                    self.pressed.set(key, true);
                    (self.callback)(key, scancode, action);
                }
            },
            glfw::Action::Repeat => {
                if self.pressed.check(key) {
                    (self.callback)(key, scancode, action);
                }
            }
            glfw::Action::Release => {
                if self.pressed.check(key) {
                    self.pressed.set(key, false);
                    (self.callback)(key, scancode, action);
                }
            }
        }
    }
}

impl Drop for KeyListener {
    fn drop(&mut self) {
        self.lose_focus();
    }
}

impl _Manager {
    fn new () -> _Manager {
        _Manager {
            key_listeners: Vec::new(),
        }
    }
}

impl Manager {
    /// Create a new manager.
    pub fn new() -> Manager {
        Manager(wrap!(_Manager::new()))
    }

    /// Feed the manager.
    pub fn emit_key(&self, key: glfw::Key, scancode: glfw::Scancode, action: glfw::Action) {
        unsafe {
            for &listener in self.0.borrow().key_listeners.iter().rev() {
                let listener = &mut (*listener);

                if listener.keys.check(key) {
                    listener.call(key, scancode, action);
                    if !listener.passtrough {
                        break;
                    }
                }
            }
        }
    }

    fn gain_key_focus(&self, focus: &mut KeyListener) {
        // If the user is currently holding a key the new listener will capture we must artificially
        // release it now otherwise the `Release` event will be captured by the wrong listener.
        // If not the extra releases will be ignored by the listeners.
        // This also makes the behavior more consistent.
        if !focus.passtrough {
            for key in &focus.keys {
                self.emit_key(key, 0, glfw::Action::Release);
            }
        }

        self.0.borrow_mut().key_listeners.push(focus as *mut _);
    }

    fn lose_key_focus(&self, focus: &mut KeyListener) {
        let focus_ptr = focus as *mut _;
        self.0.borrow_mut().key_listeners.retain(|&lptr| lptr != focus_ptr);
    }
}

impl Eq for Manager {}
impl PartialEq for Manager {
    fn eq(&self, other: &Manager) -> bool {
        &*self.0 as *const RefCell<_> == &*other.0 as *const RefCell<_>
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

const ERR_DIFF_MANAGER: &'static str = "KeyListener is already on focus in a different manager";

#[cfg(test)]
mod tests;

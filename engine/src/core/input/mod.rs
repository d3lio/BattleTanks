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
//! Events are sent to the listeners in a capturing manner, starting at the top of the stack.
//! The event is received by the listener closest to the top of the stack who has the corresponding
//! callback function set. If that listener has the `passtrough` option set then that event is also
//! passed to the listeners bellow him.

extern crate glfw;

#[macro_use]
mod mask;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub use self::mask::KeyMask;

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
    callback: Box<FnMut(glfw::Key, glfw::Scancode, glfw::Action)>,

    pressed: KeyMask,
    manager: Weak<RefCell<_Manager>>,
}

pub struct CharListener {
    passtrough: bool,
    callback: Box<FnMut(char)>,
    key_listener: KeyListener,
    manager: Weak<RefCell<_Manager>>,
}

#[derive(Debug, Clone, Copy)]
pub enum MouseEvent {
    CursorPos(f64, f64),
    CursorEnter(bool),
    Button(glfw::MouseButton, glfw::Action, glfw::Modifiers),
    Scroll(f64, f64),
}

pub struct MouseListener {
    passtrough: bool,
    callback: Box<FnMut(MouseEvent)>,
    manager: Weak<RefCell<_Manager>>,
}

struct _Manager {
    key_listeners: Vec<*mut KeyListener>,
    char_listeners: Vec<*mut CharListener>,
    mouse_listeners: Vec<*mut MouseListener>,
}

/// Input event manager.
///
/// The manager should be fed all input events from the system event queue using one
/// of the `emit_*` methods and it will distribute those events to all listeners who
/// are currently on focus.
///
/// Usually only a single instance of this class per game window should be created.
#[derive(Clone)]
pub struct Manager (Rc<RefCell<_Manager>>);

impl KeyListener {
    /// Create a new listener.
    ///
    /// The listener will capture events for the specified keys in `keys` and will trigger
    /// the callback function for each event.
    pub fn new<F> (keys: KeyMask, callback: F) -> KeyListener where
        F: FnMut(glfw::Key, glfw::Scancode, glfw::Action) + 'static
    {
        KeyListener {
            keys: keys,
            passtrough: false,
            callback: Box::new(callback),
            pressed: key_mask![],
            manager: Weak::new(),
        }
    }

    /// Create a new listener with the passtrough parameter set.
    ///
    /// With passtrough enabled events captured by this listener will also be propagated
    /// to other listeners down the chain.
    pub fn with_passtrough<F> (keys: KeyMask, callback: F) -> KeyListener where
        F: FnMut(glfw::Key, glfw::Scancode, glfw::Action) + 'static
    {
        KeyListener {
            keys: keys,
            passtrough: true,
            callback: Box::new(callback),
            pressed: key_mask![],
            manager: Weak::new(),
        }
    }

    /// Buffered input.
    ///
    /// This method pool the state from the internal event buffer, meaning that only states of the keys
    /// for which the callback function is listening are tracked.
    ///
    pub fn key_pressed(&self, key: glfw::Key) -> bool {
        self.pressed.get(key)
    }

    /// Notify the manager that the listener has gained focus.
    ///
    /// # Panics
    ///
    /// If the listener is currently on focus in a different manager.
    ///
    pub fn gain_focus(&mut self, mgr: &Manager) {
        if let Some(prev_mgr) = self.manager.upgrade() {
            if !mgr.same(&Manager(prev_mgr)) {
                panic!(ERR_DIFF_MANAGER);
            }
            return;
        }

        // If the user is currently holding a key the new listener captures we must artificially
        // release it now otherwise the `Release` event will be captured by the wrong listener.
        // If not the extra releases will be ignored by the listeners.
        // This also makes the behavior more consistent.
        if !self.passtrough {
            for key in &self.keys {
                mgr.emit_key(key, 0, glfw::Action::Release);
            }
        }

        mgr.0.borrow_mut().key_listeners.push(self as *mut _);
        self.manager = Rc::downgrade(&mgr.0);
    }

    /// Notify the manager that the listener has lost focus.
    ///
    /// The manager is stored internally, which is why it is not passed as a parameter.
    /// If the listener was not under focus this method does nothing.
    ///
    pub fn lose_focus(&mut self) {
        if let Some(mgr) = self.manager.upgrade() {
            let focus_ptr = self as *mut _;
            mgr.borrow_mut().key_listeners.retain(|&lptr| lptr != focus_ptr);
        }

        for key in &self.pressed {
            (self.callback)(key, 0, glfw::Action::Release);
        }

        self.pressed = key_mask![];
        self.manager = Weak::new();
    }

    fn call(&mut self, key: glfw::Key, scancode: glfw::Scancode, action: glfw::Action) {
        match action {
            glfw::Action::Press => {
                if !self.pressed.get(key) {
                    self.pressed.set(key, true);
                    (self.callback)(key, scancode, action);
                }
            },
            glfw::Action::Repeat => {
                if self.pressed.get(key) {
                    (self.callback)(key, scancode, action);
                }
            }
            glfw::Action::Release => {
                if self.pressed.get(key) {
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

impl CharListener {
    pub fn new<F> (callback: F) -> CharListener where
        F: FnMut(char) + 'static
    {
        CharListener {
            passtrough: false,
            callback: Box::new(callback),
            key_listener: KeyListener::new(
                key_mask![glfw::Key::Space .. glfw::Key::GraveAccent, glfw::Key::Kp0 .. glfw::Key::KpEqual],
                |_, _, _| ()),
            manager: Weak::new(),
        }
    }

    pub fn with_passtrough<F> (callback: F) -> CharListener where
        F: FnMut(char) + 'static
    {
        CharListener {
            passtrough: true,
            callback: Box::new(callback),
            key_listener: KeyListener::with_passtrough(
                key_mask![glfw::Key::Space .. glfw::Key::GraveAccent, glfw::Key::Kp0 .. glfw::Key::KpEqual],
                |_, _, _| ()),
            manager: Weak::new(),
        }
    }

    pub fn gain_focus(&mut self, mgr: &Manager) {
        if let Some(prev_mgr) = self.manager.upgrade() {
            if !mgr.same(&Manager(prev_mgr)) {
                panic!(ERR_DIFF_MANAGER);
            }
            return;
        }

        self.key_listener.gain_focus(mgr);

        mgr.0.borrow_mut().char_listeners.push(self as *mut _);
        self.manager = Rc::downgrade(&mgr.0);
    }

    pub fn lose_focus(&mut self) {
        if let Some(mgr) = self.manager.upgrade() {
            let focus_ptr = self as *mut _;
            mgr.borrow_mut().char_listeners.retain(|&lptr| lptr != focus_ptr);

            self.key_listener.lose_focus();
        }

        self.manager = Weak::new();
    }

    fn call(&mut self, codepoint: char) {
        (*self.callback)(codepoint);
    }
}

impl Drop for CharListener {
    fn drop(&mut self) {
        self.lose_focus();
    }
}

impl MouseListener {
    pub fn new<F> (callback: F) -> MouseListener where
        F: FnMut(MouseEvent) + 'static
    {
        MouseListener {
            passtrough: true,
            callback: Box::new(callback),
            manager: Weak::new(),
        }
    }

    pub fn with_passtrough<F> (callback: F) -> MouseListener where
        F: FnMut(MouseEvent) + 'static
    {
        MouseListener {
            passtrough: true,
            callback: Box::new(callback),
            manager: Weak::new(),
        }
    }

    pub fn gain_focus(&mut self, mgr: &Manager) {
        if let Some(prev_mgr) = self.manager.upgrade() {
            if !mgr.same(&Manager(prev_mgr)) {
                panic!(ERR_DIFF_MANAGER);
            }
            return;
        }

        mgr.0.borrow_mut().mouse_listeners.push(self as *mut _);
        self.manager = Rc::downgrade(&mgr.0);
    }

    pub fn lose_focus(&mut self) {
        if let Some(mgr) = self.manager.upgrade() {
            let focus_ptr = self as *mut _;
            mgr.borrow_mut().mouse_listeners.retain(|&lptr| lptr != focus_ptr);
        }

        self.manager = Weak::new();
    }

    fn call(&mut self, event: MouseEvent) {
        (*self.callback)(event);
    }
}

impl Drop for MouseListener {
    fn drop(&mut self) {
        self.lose_focus();
    }
}

impl _Manager {
    fn new () -> _Manager {
        _Manager {
            key_listeners: Vec::new(),
            char_listeners: Vec::new(),
            mouse_listeners: Vec::new(),
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
                let listener = &mut *listener;

                if listener.keys.get(key) {
                    listener.call(key, scancode, action);
                    if !listener.passtrough {
                        break;
                    }
                }
            }
        }
    }

    pub fn emit_char(&self, codepoint: char) {
        unsafe {
            for &listener in self.0.borrow().char_listeners.iter().rev() {
                let listener = &mut *listener;

                listener.call(codepoint);
                if !listener.passtrough {
                    break;
                }
            }
        }
    }

    pub fn emit_mouse_event(&self, event: MouseEvent) {
        unsafe {
            for &listener in self.0.borrow().mouse_listeners.iter().rev() {
                let listener = &mut *listener;

                listener.call(event);
                if !listener.passtrough {
                    break;
                }
            }
        }
    }

    /// Two `Manager`s are the same if they are `Rc`s to the same inner data.
    fn same(&self, other: &Manager) -> bool {
        &*self.0 as *const RefCell<_> == &*other.0 as *const RefCell<_>
    }
}

const ERR_DIFF_MANAGER: &'static str = "Listener is already on focus in a different manager";

#[cfg(test)]
mod tests;

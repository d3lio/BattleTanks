extern crate glfw;

mod key_range;

use std::cell::RefCell;
use std::io::{self, Write};
use std::ptr;
use std::rc::{Rc, Weak};

pub use self::key_range::KeyRange;

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
        self.listeners.push((keys, listener.clone()));
    }

    pub fn key_pressed(&self, key: glfw::Key) -> bool {
        let mgr_rc = match self.manager() {
            Some(mgr_rc) => mgr_rc,
            None => return false,
        };

        let mgr = mgr_rc.borrow();

        let index = mgr.key_receivers.binary_search_by(|item| item.0.cmp(&key)).unwrap();
        let listeners = &mgr.key_receivers[index];

        for l in (&listeners.1).into_iter().rev() {
            if l.1 == self as *const _ {
                unsafe {
                    return (*mgr.window).get_key(key) == glfw::Action::Press;
                }
            }

            if l.0.capture {
                break;
            }
        }

        return false;
    }

    // FIXME: temporary until `downgraded_weak` is stabilized
    fn manager(&self) -> Option<Rc<RefCell<InputManagerData>>> {
        match self.manager {
            Some(ref weak) => weak.upgrade(),
            None => None,
        }
    }
}

struct InputManagerData {
    key_receivers: Vec<(glfw::Key, Vec<(Listener, *const KeyFocus)>)>,
    window: *const glfw::Window,
}

impl InputManagerData {
    fn new() -> InputManagerData {
        let key_receivers = vec![
            (glfw::Key::Space, Vec::new()),
            (glfw::Key::Apostrophe, Vec::new()),
            (glfw::Key::Comma, Vec::new()),
            (glfw::Key::Minus, Vec::new()),
            (glfw::Key::Period, Vec::new()),
            (glfw::Key::Slash, Vec::new()),
            (glfw::Key::Num0, Vec::new()),
            (glfw::Key::Num1, Vec::new()),
            (glfw::Key::Num2, Vec::new()),
            (glfw::Key::Num3, Vec::new()),
            (glfw::Key::Num4, Vec::new()),
            (glfw::Key::Num5, Vec::new()),
            (glfw::Key::Num6, Vec::new()),
            (glfw::Key::Num7, Vec::new()),
            (glfw::Key::Num8, Vec::new()),
            (glfw::Key::Num9, Vec::new()),
            (glfw::Key::Semicolon, Vec::new()),
            (glfw::Key::Equal, Vec::new()),
            (glfw::Key::A, Vec::new()),
            (glfw::Key::B, Vec::new()),
            (glfw::Key::C, Vec::new()),
            (glfw::Key::D, Vec::new()),
            (glfw::Key::E, Vec::new()),
            (glfw::Key::F, Vec::new()),
            (glfw::Key::G, Vec::new()),
            (glfw::Key::H, Vec::new()),
            (glfw::Key::I, Vec::new()),
            (glfw::Key::J, Vec::new()),
            (glfw::Key::K, Vec::new()),
            (glfw::Key::L, Vec::new()),
            (glfw::Key::M, Vec::new()),
            (glfw::Key::N, Vec::new()),
            (glfw::Key::O, Vec::new()),
            (glfw::Key::P, Vec::new()),
            (glfw::Key::Q, Vec::new()),
            (glfw::Key::R, Vec::new()),
            (glfw::Key::S, Vec::new()),
            (glfw::Key::T, Vec::new()),
            (glfw::Key::U, Vec::new()),
            (glfw::Key::V, Vec::new()),
            (glfw::Key::W, Vec::new()),
            (glfw::Key::X, Vec::new()),
            (glfw::Key::Y, Vec::new()),
            (glfw::Key::Z, Vec::new()),
            (glfw::Key::LeftBracket, Vec::new()),
            (glfw::Key::Backslash, Vec::new()),
            (glfw::Key::RightBracket, Vec::new()),
            (glfw::Key::GraveAccent, Vec::new()),
            (glfw::Key::World1, Vec::new()),
            (glfw::Key::World2, Vec::new()),
            (glfw::Key::Escape, Vec::new()),
            (glfw::Key::Enter, Vec::new()),
            (glfw::Key::Tab, Vec::new()),
            (glfw::Key::Backspace, Vec::new()),
            (glfw::Key::Insert, Vec::new()),
            (glfw::Key::Delete, Vec::new()),
            (glfw::Key::Right, Vec::new()),
            (glfw::Key::Left, Vec::new()),
            (glfw::Key::Down, Vec::new()),
            (glfw::Key::Up, Vec::new()),
            (glfw::Key::PageUp, Vec::new()),
            (glfw::Key::PageDown, Vec::new()),
            (glfw::Key::Home, Vec::new()),
            (glfw::Key::End, Vec::new()),
            (glfw::Key::CapsLock, Vec::new()),
            (glfw::Key::ScrollLock, Vec::new()),
            (glfw::Key::NumLock, Vec::new()),
            (glfw::Key::PrintScreen, Vec::new()),
            (glfw::Key::Pause, Vec::new()),
            (glfw::Key::F1, Vec::new()),
            (glfw::Key::F2, Vec::new()),
            (glfw::Key::F3, Vec::new()),
            (glfw::Key::F4, Vec::new()),
            (glfw::Key::F5, Vec::new()),
            (glfw::Key::F6, Vec::new()),
            (glfw::Key::F7, Vec::new()),
            (glfw::Key::F8, Vec::new()),
            (glfw::Key::F9, Vec::new()),
            (glfw::Key::F10, Vec::new()),
            (glfw::Key::F11, Vec::new()),
            (glfw::Key::F12, Vec::new()),
            (glfw::Key::F13, Vec::new()),
            (glfw::Key::F14, Vec::new()),
            (glfw::Key::F15, Vec::new()),
            (glfw::Key::F16, Vec::new()),
            (glfw::Key::F17, Vec::new()),
            (glfw::Key::F18, Vec::new()),
            (glfw::Key::F19, Vec::new()),
            (glfw::Key::F20, Vec::new()),
            (glfw::Key::F21, Vec::new()),
            (glfw::Key::F22, Vec::new()),
            (glfw::Key::F23, Vec::new()),
            (glfw::Key::F24, Vec::new()),
            (glfw::Key::F25, Vec::new()),
            (glfw::Key::Kp0, Vec::new()),
            (glfw::Key::Kp1, Vec::new()),
            (glfw::Key::Kp2, Vec::new()),
            (glfw::Key::Kp3, Vec::new()),
            (glfw::Key::Kp4, Vec::new()),
            (glfw::Key::Kp5, Vec::new()),
            (glfw::Key::Kp6, Vec::new()),
            (glfw::Key::Kp7, Vec::new()),
            (glfw::Key::Kp8, Vec::new()),
            (glfw::Key::Kp9, Vec::new()),
            (glfw::Key::KpDecimal, Vec::new()),
            (glfw::Key::KpDivide, Vec::new()),
            (glfw::Key::KpMultiply, Vec::new()),
            (glfw::Key::KpSubtract, Vec::new()),
            (glfw::Key::KpAdd, Vec::new()),
            (glfw::Key::KpEnter, Vec::new()),
            (glfw::Key::KpEqual, Vec::new()),
            (glfw::Key::LeftShift, Vec::new()),
            (glfw::Key::LeftControl, Vec::new()),
            (glfw::Key::LeftAlt, Vec::new()),
            (glfw::Key::LeftSuper, Vec::new()),
            (glfw::Key::RightShift, Vec::new()),
            (glfw::Key::RightControl, Vec::new()),
            (glfw::Key::RightAlt, Vec::new()),
            (glfw::Key::RightSuper, Vec::new()),
            (glfw::Key::Menu, Vec::new()),
        ];

        InputManagerData {
            key_receivers: key_receivers,
            window: ptr::null(),
        }
    }
}

pub struct InputManager(Rc<RefCell<InputManagerData>>);

impl InputManager {
    pub fn new() -> InputManager {
        InputManager(Rc::new(RefCell::new(InputManagerData::new())))
    }

    #[allow(unused_must_use)]
    pub fn gain_focus(&self, focus: &mut KeyFocus) {
        if let Some(mgr) = focus.manager() {
            InputManager(mgr).lose_focus(focus);
        }

        let mut mgr = self.0.borrow_mut();
        let focus_ptr = focus as *const _;

        for listener in &focus.listeners {
            for key in &listener.0 {
                match mgr.key_receivers.binary_search_by(|item| (item.0 as i32).cmp(&key)) {
                    Ok(index) => mgr.key_receivers[index].1.push((listener.1.clone(), focus_ptr)),
                    Err(_) => { writeln!(&mut io::stderr(), "{} is not a valid value for glfw::Key", key); },
                }
            }
        }

        focus.manager = Some(Rc::downgrade(&self.0));
    }

    #[allow(unused_must_use)]
    pub fn lose_focus(&self, focus: &mut KeyFocus) {
        let mut mgr = self.0.borrow_mut();
        let focus_ptr = focus as *const _;

        for listener in &focus.listeners {
            for key in &listener.0 {
                match mgr.key_receivers.binary_search_by(|item| (item.0 as i32).cmp(&key)) {
                    // TODO: optimize retain
                    Ok(index) => mgr.key_receivers[index].1.retain(|item| item.1 != focus_ptr),
                    Err(_) => { writeln!(&mut io::stderr(), "{} is not a valid value for glfw::Key", key); },
                }
            }
        }

        focus.manager = None;
    }
}

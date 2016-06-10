extern crate glfw;

use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

use core::input::{Listener, KeyFocus};

struct KeyReceiver {
    key: glfw::Key,
    listeners: Vec<(Listener, *const KeyFocus)>,
}

pub struct InputManagerData {
    key_receivers: Vec<KeyReceiver>,
    window: *const glfw::Window,
}

impl InputManagerData {
    // FIXME: No lifetime checks.
    fn new(window: &glfw::Window) -> InputManagerData {
        let key_receivers = vec![
            KeyReceiver{key: glfw::Key::Space, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Apostrophe, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Comma, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Minus, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Period, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Slash, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Num0, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Num1, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Num2, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Num3, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Num4, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Num5, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Num6, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Num7, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Num8, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Num9, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Semicolon, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Equal, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::A, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::B, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::C, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::D, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::E, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::G, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::H, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::I, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::J, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::K, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::L, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::M, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::N, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::O, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::P, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Q, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::R, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::S, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::T, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::U, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::V, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::W, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::X, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Y, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Z, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::LeftBracket, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Backslash, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::RightBracket, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::GraveAccent, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::World1, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::World2, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Escape, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Enter, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Tab, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Backspace, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Insert, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Delete, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Right, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Left, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Down, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Up, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::PageUp, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::PageDown, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Home, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::End, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::CapsLock, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::ScrollLock, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::NumLock, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::PrintScreen, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Pause, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F1, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F2, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F3, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F4, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F5, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F6, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F7, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F8, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F9, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F10, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F11, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F12, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F13, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F14, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F15, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F16, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F17, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F18, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F19, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F20, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F21, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F22, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F23, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F24, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::F25, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Kp0, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Kp1, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Kp2, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Kp3, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Kp4, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Kp5, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Kp6, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Kp7, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Kp8, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Kp9, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::KpDecimal, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::KpDivide, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::KpMultiply, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::KpSubtract, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::KpAdd, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::KpEnter, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::KpEqual, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::LeftShift, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::LeftControl, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::LeftAlt, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::LeftSuper, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::RightShift, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::RightControl, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::RightAlt, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::RightSuper, listeners: Vec::new()},
            KeyReceiver{key: glfw::Key::Menu, listeners: Vec::new()},
        ];

        InputManagerData {
            key_receivers: key_receivers,
            window: window as *const _,
        }
    }

    pub fn key_unbuffered(&self, focus: &KeyFocus, key: glfw::Key) -> bool {
        let index = self.key_receivers.binary_search_by(|item| item.key.cmp(&key)).unwrap();
        let receivers = &self.key_receivers[index];

        for &(ref listener, ptr) in (&receivers.listeners).into_iter().rev() {
            if ptr == focus as *const _ {
                return unsafe { (*self.window).get_key(key) == glfw::Action::Press };
            }

            if listener.capture {
                break;
            }
        }

        return false;
    }
}

pub struct InputManager(Rc<RefCell<InputManagerData>>);

impl InputManager {
    pub fn new(window: &glfw::Window) -> InputManager {
        InputManager(wrap!(InputManagerData::new(window)))
    }

    #[allow(unused_must_use)]
    pub fn gain_focus(&self, focus: &mut KeyFocus) {
        if let Some(mgr) = focus.manager() {
            InputManager(mgr).lose_focus(focus);
        }

        let mut mgr = self.0.borrow_mut();
        let focus_ptr = focus as *const _;

        for &(ref keys, ref listener) in &focus.listeners {
            for key in keys {
                match mgr.key_receivers.binary_search_by(|item| (item.key as i32).cmp(&key)) {
                    Ok(index) => mgr.key_receivers[index].listeners.push((listener.clone(), focus_ptr)),
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

        for &(ref keys, _) in &focus.listeners {
            for key in keys {
                match mgr.key_receivers.binary_search_by(|item| (item.key as i32).cmp(&key)) {
                    Ok(index) => mgr.key_receivers[index].listeners.retain(|&(_, ptr)| ptr != focus_ptr),
                    Err(_) => { writeln!(&mut io::stderr(), "{} is not a valid value for glfw::Key", key); },
                }
            }
        }

        focus.manager = None;
    }

    pub fn emit_key(&self, key: glfw::Key, scancode: glfw::Scancode, action: glfw::Action) {
        let mgr = self.0.borrow();

        let index = mgr.key_receivers.binary_search_by(|item| item.key.cmp(&key)).unwrap();

        for &(ref listener, _) in (&mgr.key_receivers[index].listeners).into_iter().rev() {
            listener.call(key, scancode, action);

            if listener.capture {
                break;
            }
        }
    }
}

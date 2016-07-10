extern crate glfw;

use std::iter::IntoIterator;
use std::ops::Range;

// TODO: use a bitmap

/// A binary mask for the keys of the glfw::Key enum.
///
/// A `KeyMask` object can also be created using the `key_mask!` macro.
pub struct KeyMask {
    mask: [bool; GLFW_KEY_COUNT],
}

// derive(Clone, Copy) fails because they are not defined for [bool; 120]
impl Copy for KeyMask {}
impl Clone for KeyMask {
    fn clone(&self) -> KeyMask {
        KeyMask {
            mask: self.mask
        }
    }
}

impl KeyMask {
    /// Create a new mask with all bits set to false.
    pub fn new() -> KeyMask {
        KeyMask {
            mask: [false; GLFW_KEY_COUNT],
        }
    }

    /// Set the bit associated with a key.
    #[inline]
    pub fn set(&mut self, key: glfw::Key, val: bool) {
        self.mask[GLFW_KEY_MAP[key as usize] as usize] = val;
    }

    /// Set the bits associated with a range of keys.
    ///
    /// `range` is inclusive, that is a range `Key::A .. Key::Z` will include `Key::A` and `Key::Z`.
    pub fn set_range(&mut self, range: Range<glfw::Key>, val: bool) {
        for key in range.start as usize .. range.end as usize + 1 {
            let index = GLFW_KEY_MAP[key];
            if index != -1 {
                self.mask[index as usize] = val;
            }
        }
    }

    /// Get the bit associated with a key.
    #[inline]
    pub fn get(&self, key: glfw::Key) -> bool {
        self.mask[GLFW_KEY_MAP[key as usize] as usize]
    }
}

impl<'a> IntoIterator for &'a KeyMask {
    type Item = glfw::Key;
    type IntoIter = Box<Iterator<Item=glfw::Key> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new((0 .. GLFW_KEY_COUNT).filter(move |&i| self.mask[i] == true).map(|i| GLFW_REV_KEY_MAP[i]))
    }
}

/// Creates a `core::input::KeyMap` object.
///
/// # Examples
/// ```
/// #[macro_use(key_mask)]
/// extern crate engine;
/// extern crate glfw;
/// use self::glfw::Key;
///
/// # fn main() {
/// // has the bits for keys F1, F2 and F3 set
/// let km1 = key_mask![Key::F1, Key::F2, Key::F3];
///
/// // has the bits for keys Space, Enter, [A; Z] (inclusive) and [0; 9] (inclusive) set
/// let km2 = key_mask![Key::Space, Key::Enter; Key::A .. Key::Z, Key::Num0 .. Key::Num9];
/// # }
/// ```
#[macro_export]
macro_rules! key_mask {
    () => (
        KeyMask::new()
    );

    ( $( $key:expr ),* ) => (
        key_mask![$( $key ),* ;]
    );

    ( $( $key:expr ),* ; $( $range:expr ),* ) => ({
        use $crate::core::input::KeyMask;

        let mut mask = KeyMask::new();
        $( mask.set($key, true); )*
        $( mask.set_range($range, true); )*

        mask
    })
}

// TODO: see if we can make this less hardcoded by using `const fn`.

/// A mapping of the glfw::Key enum to continuous numbers
const GLFW_KEY_MAP: [i16; glfw::ffi::KEY_LAST as usize + 1] = [
     -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
     -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,   0,  -1,  -1,  -1,  -1,  -1,  -1,   1,
     -1,  -1,  -1,  -1,   2,   3,   4,   5,   6,   7,   8,   9,  10,  11,  12,  13,  14,  15,  -1,  16,
     -1,  17,  -1,  -1,  -1,  18,  19,  20,  21,  22,  23,  24,  25,  26,  27,  28,  29,  30,  31,  32,
     33,  34,  35,  36,  37,  38,  39,  40,  41,  42,  43,  44,  45,  46,  -1,  -1,  47,  -1,  -1,  -1,
     -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
     -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
     -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
     -1,  48,  49,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
     -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
     -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
     -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
     -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  50,  51,  52,  53,
     54,  55,  56,  57,  58,  59,  60,  61,  62,  63,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,  -1,
     64,  65,  66,  67,  68,  -1,  -1,  -1,  -1,  -1,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,
     79,  80,  81,  82,  83,  84,  85,  86,  87,  88,  89,  90,  91,  92,  93,  -1,  -1,  -1,  -1,  -1,
     94,  95,  96,  97,  98,  99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,  -1,  -1,  -1,
    111, 112, 113, 114, 115, 116, 117, 118, 119,
];

/// A mapping of continuous numbers to the glfw::Key enum
const GLFW_REV_KEY_MAP: [glfw::Key; GLFW_KEY_COUNT] = [
    glfw::Key::Space,
    glfw::Key::Apostrophe,
    glfw::Key::Comma,
    glfw::Key::Minus,
    glfw::Key::Period,
    glfw::Key::Slash,
    glfw::Key::Num0,
    glfw::Key::Num1,
    glfw::Key::Num2,
    glfw::Key::Num3,
    glfw::Key::Num4,
    glfw::Key::Num5,
    glfw::Key::Num6,
    glfw::Key::Num7,
    glfw::Key::Num8,
    glfw::Key::Num9,
    glfw::Key::Semicolon,
    glfw::Key::Equal,
    glfw::Key::A,
    glfw::Key::B,
    glfw::Key::C,
    glfw::Key::D,
    glfw::Key::E,
    glfw::Key::F,
    glfw::Key::G,
    glfw::Key::H,
    glfw::Key::I,
    glfw::Key::J,
    glfw::Key::K,
    glfw::Key::L,
    glfw::Key::M,
    glfw::Key::N,
    glfw::Key::O,
    glfw::Key::P,
    glfw::Key::Q,
    glfw::Key::R,
    glfw::Key::S,
    glfw::Key::T,
    glfw::Key::U,
    glfw::Key::V,
    glfw::Key::W,
    glfw::Key::X,
    glfw::Key::Y,
    glfw::Key::Z,
    glfw::Key::LeftBracket,
    glfw::Key::Backslash,
    glfw::Key::RightBracket,
    glfw::Key::GraveAccent,
    glfw::Key::World1,
    glfw::Key::World2,
    glfw::Key::Escape,
    glfw::Key::Enter,
    glfw::Key::Tab,
    glfw::Key::Backspace,
    glfw::Key::Insert,
    glfw::Key::Delete,
    glfw::Key::Right,
    glfw::Key::Left,
    glfw::Key::Down,
    glfw::Key::Up,
    glfw::Key::PageUp,
    glfw::Key::PageDown,
    glfw::Key::Home,
    glfw::Key::End,
    glfw::Key::CapsLock,
    glfw::Key::ScrollLock,
    glfw::Key::NumLock,
    glfw::Key::PrintScreen,
    glfw::Key::Pause,
    glfw::Key::F1,
    glfw::Key::F2,
    glfw::Key::F3,
    glfw::Key::F4,
    glfw::Key::F5,
    glfw::Key::F6,
    glfw::Key::F7,
    glfw::Key::F8,
    glfw::Key::F9,
    glfw::Key::F10,
    glfw::Key::F11,
    glfw::Key::F12,
    glfw::Key::F13,
    glfw::Key::F14,
    glfw::Key::F15,
    glfw::Key::F16,
    glfw::Key::F17,
    glfw::Key::F18,
    glfw::Key::F19,
    glfw::Key::F20,
    glfw::Key::F21,
    glfw::Key::F22,
    glfw::Key::F23,
    glfw::Key::F24,
    glfw::Key::F25,
    glfw::Key::Kp0,
    glfw::Key::Kp1,
    glfw::Key::Kp2,
    glfw::Key::Kp3,
    glfw::Key::Kp4,
    glfw::Key::Kp5,
    glfw::Key::Kp6,
    glfw::Key::Kp7,
    glfw::Key::Kp8,
    glfw::Key::Kp9,
    glfw::Key::KpDecimal,
    glfw::Key::KpDivide,
    glfw::Key::KpMultiply,
    glfw::Key::KpSubtract,
    glfw::Key::KpAdd,
    glfw::Key::KpEnter,
    glfw::Key::KpEqual,
    glfw::Key::LeftShift,
    glfw::Key::LeftControl,
    glfw::Key::LeftAlt,
    glfw::Key::LeftSuper,
    glfw::Key::RightShift,
    glfw::Key::RightControl,
    glfw::Key::RightAlt,
    glfw::Key::RightSuper,
    glfw::Key::Menu,
];

/// Number of entries in the glfw enum
const GLFW_KEY_COUNT: usize = 120;

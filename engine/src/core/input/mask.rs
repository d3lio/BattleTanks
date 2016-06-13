extern crate glfw;

use std::ops::Range;

/// A binary mask for the keys of the glfw::Key enum.
///
/// A `KeyMask` object can also be created using the `key_mask!` macro
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
    pub fn check(&self, key: glfw::Key) -> bool {
        self.mask[GLFW_KEY_MAP[key as usize] as usize]
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

/// A mapping of the glfw::Key enum to continuous numbers
const GLFW_KEY_MAP: [isize; glfw::ffi::KEY_LAST as usize + 1] = [
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

/// Number of entries in the glfw enum
const GLFW_KEY_COUNT: usize = 120;

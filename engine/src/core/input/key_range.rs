extern crate glfw;

use std::cmp;
use std::ops::Range;
use std::slice;

/// A set of Glfw keycodes.
///
/// Instances of this class are designed to be created by the `key_range` macro, which
/// uses the constructor method internally.
///
/// Internally it is represented by a vector of sorted non-overlapping ranges.
///
///
#[derive(Debug, Clone)]
pub struct KeyRange(pub Vec<Range<i32>>);

pub struct Iter<'a> {
    ranges: slice::Iter<'a, Range<i32>>,
    curr_range: Range<i32>,
}

impl KeyRange {
    /// Construct a `KeyRange` from a vector of ranges.
    ///
    /// This constructor takes care of sorting the ranges, merging overlapping ranges and removing invalid ones.
    ///
    /// Note that unlike the `key_range!` macro, this constructor accepts the standard Rust ranges, which are
    /// half-open. Artificially incrementing the range end by 1 is required. <br>
    /// There is also no support for individual keys - use a range `key .. key+1` instead.
    pub fn new(mut vec: Vec<Range<i32>>) -> KeyRange {
        vec.sort_by(|a, b| {
            if a.start != b.start {
                i32::cmp(&a.start, &b.start)
            } else {
                i32::cmp(&a.end, &b.end)
            }
        });

        let mut l = 0;
        let mut r = 1;

        while r < vec.len() {
            if vec[l].end >= vec[r].start {
                vec[l].end = cmp::max(vec[l].end, vec[r].end);
                vec[r].start = vec[r].end;
                r += 1;
            }
            else {
                l = r;
                r += 1;
            }
        }

        vec.retain(|item| item.start < item.end);
        vec.shrink_to_fit();

        KeyRange(vec)
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        loop {
            if let Some(val) = self.curr_range.next() {
                return Some(val);
            }

            match self.ranges.next() {
                Some(range) => self.curr_range = range.clone(),
                None => return None,
            }
        }
    }
}

impl<'a> IntoIterator for &'a KeyRange {
    type Item = i32;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        Iter {
            ranges: self.0.iter(),
            curr_range: 0..0,
        }
    }
}

/// Creates an instance of `core::input::KeyRange`.
///
/// Syntax: A comma separated list of individual keys, followed by `;`, followed by
/// a comma separated list of ranges of keys.
///
/// Key ranges are inclusive.
/// It is valid to specify a range of keys only if their corresponding values in the
/// [Glfw keys enum](http://www.glfw.org/docs/latest/group__keys.html) are subsequent numbers.
///
/// See the [KeyRange documentation](./core/input/struct.KeyRange.html) for more info.
///
/// # Example
/// ```
/// #[macro_use(key_range)]
/// extern crate engine;
/// extern crate glfw;
/// use self::glfw::Key;
///
/// # fn main() {
/// // contains the keys Space, Enter, [A; Z] (inclusive) and [0; 9] (inclusive)
/// let range = key_range![Key::Space, Key::Enter; Key::A .. Key::Z, Key::Num0 .. Key::Num9];
/// # }
/// ```
///
#[macro_export]
macro_rules! key_range {
    ( $( $key:expr ),* ; $( $range:expr ),* ) => ({
        use $crate::core::input::KeyRange;
        use std::ops::Range;

        KeyRange::new(vec![
            $( Range{start: $key as i32, end: $key as i32 + 1}, )*
            $( Range{start: $range.start as i32, end: $range.end as i32 + 1}, )*
        ])
    })
}

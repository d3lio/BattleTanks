extern crate glfw;

use std::ops::Range;
use std::slice;

#[derive(Debug, Clone)]
pub struct KeyRange {
    ranges: Vec<Range<i32>>,
}

pub struct Iter<'a> {
    ranges: slice::Iter<'a, Range<i32>>,
    curr_range: Range<i32>,
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
            ranges: self.ranges.iter(),
            curr_range: 0..0,
        }
    }
}

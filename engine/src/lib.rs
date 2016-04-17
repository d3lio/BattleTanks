pub mod core;
pub mod gliw;
pub mod math;

/// Global macro for wrapping objects in Rc + RefCell.
#[macro_export]
macro_rules! wrap {
    ($x: expr) => {
        std::rc::Rc::new(std::cell::RefCell::new($x));
    }
}

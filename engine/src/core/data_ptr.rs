/// Powerful raw pointer wrapper.
///
/// It has the semantics of the C `void*` type but since it's a tuple struct it
/// should not be confused with `libc::c_void`, ZSTs or empty types.
///
/// **This structure hides great unsafety because it can dereference an invalid raw pointer
/// and thus it should be used with caution.**
///
/// With that out of the way here is what you can do with it if used properly:
///
/// * Transmute data
/// * Apply C style memory inheritance
/// * Or just boring data transfer avoiding rust borrowing and lifetime rules
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Data(*mut ());

impl Data {
    /// Creates a new data structure from a mutable reference.
    ///
    /// This function is perfectly safe unlike `Data::to()`.
    #[inline(always)]
    pub fn from<T>(data_ref: &mut T) -> Data {
        Data(data_ref as *mut T as *mut ())
    }

    /// Extract the inner data as a mutable reference.
    ///
    /// This function is **unsafe** but due to the frequent usage the unsafety is hidden.
    /// Also it can create a situation such that there could be multiple mutable and immutable
    /// references to the same data which is against rust laws and thus should be used with caution.
    /// For example do not attempt to manually drop the referenced data.
    ///
    /// A couple of things to note when calling this function:
    ///
    /// * If a pointer to invalid data is stored in the structure it is considered undefined behaviour.
    /// * If a null pointer is stored then the current thread panics.
    #[inline(always)]
    pub fn to<T>(&self) -> &mut T {
        if self.is_null() {
            panic!("Dereferencing a null pointer!");
        }

        unsafe { &mut*(self.0 as *mut T) }
    }

    /// Creates a new data structure with a null pointer.
    ///
    /// Using `Data::to()` on the result from this method will panic.
    #[inline(always)]
    pub fn null() -> Data {
        Data(0 as *mut ())
    }

    /// Checks if the underlying pointer is null.
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

#[cfg(test)]
mod tests {
    use super::Data;

    #[test]
    fn is_null() {
        assert!(Data::null().is_null());
    }

    #[test]
    #[should_panic]
    fn deref_null() {
        *Data::null().to::<u32>();
    }

    #[test]
    fn from_to() {
        assert_eq!(*Data::from(&mut 5u32).to::<u32>(), 5u32);
    }
}

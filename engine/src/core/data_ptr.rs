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
#[derive(Copy, Clone)]
pub struct Data(*mut ());

impl Data {
    /// Creates a new data structure from a mutable reference.
    ///
    /// This function is perfectrly safe unlike `Data::to`.
    #[inline]
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
    #[inline]
    pub fn to<T>(&self) -> &mut T {
        if self.0 as i32 == 0 {
            // Highly unlikely but it can happen if the initial reference
            // was created from a null pointer.
            panic!("Dereferencing a null pointer!");
        }

        unsafe { &mut*(self.0 as *mut T) }
    }
}

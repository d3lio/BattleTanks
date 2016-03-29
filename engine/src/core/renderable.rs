use core::Camera;

/// Specifies if an object is renderable.
pub trait Renderable {
    /// Specifies the order in which the objects will be rendered in a `Scene`.
    fn priority(&self) -> u32 {
        return 0;
    }

    /// Draw call.
    fn draw(&self, camera: &Camera);
}

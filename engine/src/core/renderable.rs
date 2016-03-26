/// Specifies if an object is renderable.
pub trait Renderable {
    fn priority(&self) -> u32;
    fn draw(&self);
}

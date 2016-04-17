extern crate cgmath;

use self::cgmath::Matrix4;

use super::camera::Camera;

/// Determines if an object is renderable and defines its properties.
pub trait Renderable {
    /// Specifies the order in which the objects will be rendered in a `Scene`.
    ///
    /// Defaults to `0`.
    fn priority(&self) -> u32 {
        return 0;
    }

    /// Get the renderable's model matrix.
    fn model_matrix(&self) -> Matrix4<f32>;

    /// Draw call.
    fn draw(&self, draw_space: Matrix4<f32>, camera: &Camera);
}

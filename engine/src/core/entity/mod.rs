extern crate cgmath;

pub mod cuboid;

use self::cgmath::{Point3, Vector3, Matrix4, Quaternion};

/// Entity trait representing basic transformations.
pub trait Entity {
    /// Get the position.
    fn position(&self) -> Point3<f32>;

    /// Get the orientation.
    fn orientation(&self) -> Quaternion<f32>;

    /// Get the scale.
    fn scale(&self) -> f32;

    // Translate the entity `n` units towards it's orientation direction.
    //
    // Negative value indicates backwards translation.
    // fn move_by(&mut self, units: f32);

    /// Teleport the entity to the given position.
    fn move_to(&mut self, position: Point3<f32>);

    /// Rotate the entity.
    fn look_at(&mut self, dir: Vector3<f32>, up: Vector3<f32>);

    /// Relative multiplicative scale.
    fn scale_by(&mut self, units: f32);

    /// Non relative scale.
    fn scale_to(&mut self, units: f32);

    /// Get the entity's model matrix.
    fn model_matrix(&self) -> Matrix4<f32>;
}

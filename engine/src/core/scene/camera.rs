extern crate cgmath;

use self::cgmath::{
    Point3, Vector3,
    Matrix4, SquareMatrix,
    Angle, Deg,
};

#[derive(Copy, Clone)]
/// Holds view and projection matrices.
///
/// See `Scene`.
pub struct Camera {
    view_matrix: Matrix4<f32>,
    proj_matrix: Matrix4<f32>,
    // Reduces draw call computations
    vp_matrix: Matrix4<f32>
}

impl Camera {
    /// Create a new camera.
    pub fn new() -> Camera {
        return Camera {
            view_matrix: Matrix4::identity(),
            proj_matrix: Matrix4::identity(),
            vp_matrix: Matrix4::identity()
        };
    }

    /// Create a new `Camera` from view and projection matrices.
    pub fn from_matrices(view_matrix: Matrix4<f32>, proj_matrix: Matrix4<f32>) -> Camera {
        return Camera {
            view_matrix: view_matrix,
            proj_matrix: proj_matrix,
            vp_matrix: proj_matrix * view_matrix
        };
    }

    /// Get VP matrix.
    pub fn vp_matrix(&self) -> Matrix4<f32> {
        return self.vp_matrix;
    }

    /// Update the view matrix.
    pub fn look_at(&mut self, eye: Point3<f32>, center: Point3<f32>, up: Vector3<f32>) {
        self.view_matrix = Matrix4::look_at(eye, center, up);
        self.vp_matrix = self.proj_matrix * self.view_matrix;
    }

    /// Update the projection matrix.
    pub fn perspective(&mut self, fovy: f32, aspect: f32, near: f32, far: f32) {
        self.proj_matrix = cgmath::perspective(Deg::new(fovy), aspect, near, far);
        self.vp_matrix = self.proj_matrix * self.view_matrix;
    }
}

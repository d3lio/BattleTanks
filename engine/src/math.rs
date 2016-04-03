//! Temporary math module.

extern crate cgmath;

use self::cgmath::{Matrix4, Quaternion};

/// Rotation Mat4 from Quat.
pub trait RotMat {
    fn from_quat(q: &Quaternion<f32>) -> Matrix4<f32>;
}

impl RotMat for Matrix4<f32> {
    fn from_quat(q: &Quaternion<f32>) -> Self {
        let q_ref: &[f32; 4] = q.as_ref();

        let xx: f32 = 2.0*q_ref[1]*q_ref[1];
        let yy: f32 = 2.0*q_ref[2]*q_ref[2];
        let zz: f32 = 2.0*q_ref[3]*q_ref[3];

        let xy: f32 = 2.0*q_ref[1]*q_ref[2];
        let yz: f32 = 2.0*q_ref[2]*q_ref[3];
        let zx: f32 = 2.0*q_ref[3]*q_ref[1];

        let wx: f32 = 2.0*q_ref[0]*q_ref[1];
        let wy: f32 = 2.0*q_ref[0]*q_ref[2];
        let wz: f32 = 2.0*q_ref[0]*q_ref[3];

        // Col major
        return Matrix4::new(
            1.0-yy-zz, xy + wz  , zx - wy  , 0.0,
            xy - wz  , 1.0-xx-zz, yz + wx  , 0.0,
            zx + wy  , yz - wx  , 1.0-xx-yy, 0.0,
            0.0      , 0.0      , 0.0      , 1.0,
        );
    }
}

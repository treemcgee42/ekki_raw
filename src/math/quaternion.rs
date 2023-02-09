use std::ops::Mul;

use cgmath::{InnerSpace, Rotation3};

use super::{vector::Vector3, Float, Radians};

#[derive(Clone, Copy)]
pub struct Quaternion {
    pub(super) internal: cgmath::Quaternion<Float>,
}

impl Quaternion {
    pub fn new(w: Float, xi: Float, yj: Float, zk: Float) -> Self {
        Self {
            internal: cgmath::Quaternion::new(w, xi, yj, zk),
        }
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    pub fn rotation_from_axis_angle<A>(axis: Vector3, angle: A) -> Self
    where
        A: Into<Radians>,
    {
        let rad: Radians = angle.into();
        Self {
            internal: cgmath::Quaternion::from_axis_angle(axis.internal, cgmath::Rad(rad.0)),
        }
    }

    pub fn normalize(self) -> Self {
        Self {
            internal: self.internal.normalize(),
        }
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            internal: self.internal * rhs.internal,
        }
    }
}

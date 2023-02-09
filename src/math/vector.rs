use std::ops::{Add, Neg};

use super::point::Point3;
use super::Float;

pub struct Vector3 {
    pub(super) internal: cgmath::Vector3<Float>,
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            internal: self.internal + rhs.internal,
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            internal: -self.internal,
        }
    }
}

impl From<Point3> for Vector3 {
    fn from(p: Point3) -> Self {
        Self {
            internal: cgmath::EuclideanSpace::to_vec(p.internal),
        }
    }
}

impl Vector3 {
    pub fn unit_y() -> Self {
        Self {
            internal: cgmath::Vector3::unit_y(),
        }
    }

    pub fn dot(v1: Self, v2: Self) -> Float {
        cgmath::dot(v1.internal, v2.internal)
    }
}

pub struct Vector4 {
    pub(super) internal: cgmath::Vector4<Float>,
}

impl Vector4 {
    pub fn truncate(self) -> Vector3 {
        Vector3 {
            internal: self.internal.truncate(),
        }
    }
}

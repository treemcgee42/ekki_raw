use std::{hash::Hash, ops::Sub};

use super::{vector::Vector3, Float};

#[derive(Clone, Copy)]
pub struct Point3 {
    pub(super) internal: cgmath::Point3<Float>,
}

impl Sub for Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs_as_vec = cgmath::EuclideanSpace::to_vec(self.internal);
        let rhs_as_vec = cgmath::EuclideanSpace::to_vec(rhs.internal);

        Vector3 {
            internal: lhs_as_vec - rhs_as_vec,
        }
    }
}

impl Sub for &Point3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        self.clone() - rhs.clone()
    }
}

impl Into<[f32; 3]> for Point3 {
    fn into(self) -> [f32; 3] {
        self.internal.into()
    }
}

impl Point3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self {
            internal: cgmath::Point3::new(x, y, z),
        }
    }

    pub fn origin() -> Self {
        Self {
            internal: cgmath::Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn to_vec3(&self) -> Vector3 {
        Vector3 {
            internal: cgmath::EuclideanSpace::to_vec(self.internal),
        }
    }
}

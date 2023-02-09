use std::ops::Mul;

use cgmath::SquareMatrix;

use super::quaternion::Quaternion;
use super::vector::{Vector3, Vector4};
use super::{Float, Radians};

#[derive(Clone, Copy)]
pub struct Matrix4 {
    internal: cgmath::Matrix4<Float>,
}

impl From<Quaternion> for Matrix4 {
    fn from(q: Quaternion) -> Self {
        Self {
            internal: cgmath::Matrix4::from(q.internal),
        }
    }
}

impl Matrix4 {
    pub const fn new(
        c0r0: Float,
        c0r1: Float,
        c0r2: Float,
        c0r3: Float,
        c1r0: Float,
        c1r1: Float,
        c1r2: Float,
        c1r3: Float,
        c2r0: Float,
        c2r1: Float,
        c2r2: Float,
        c2r3: Float,
        c3r0: Float,
        c3r1: Float,
        c3r2: Float,
        c3r3: Float,
    ) -> Self {
        Self {
            internal: cgmath::Matrix4::new(
                c0r0, c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2, c2r3, c3r0, c3r1,
                c3r2, c3r3,
            ),
        }
    }

    pub fn from_translation(t: Vector3) -> Self {
        Self {
            internal: cgmath::Matrix4::from_translation(t.internal),
        }
    }

    pub fn create_perspective<A>(
        vertical_fov: A,
        aspect_ratio: Float,
        near_clipping_z: Float,
        far_clipping_z: Float,
    ) -> Self
    where
        A: Into<Radians>,
    {
        Self {
            internal: cgmath::perspective(
                cgmath::Rad(vertical_fov.into().0),
                aspect_ratio,
                near_clipping_z,
                far_clipping_z,
            ),
        }
    }

    pub fn y(&self) -> Vector4 {
        Vector4 {
            internal: self.internal.y,
        }
    }
}

impl Into<[[Float; 4]; 4]> for Matrix4 {
    fn into(self) -> [[Float; 4]; 4] {
        self.internal.into()
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        Self {
            internal: self.internal * rhs.internal,
        }
    }
}

pub struct Matrix3 {
    internal: cgmath::Matrix3<Float>,
}

impl From<Quaternion> for Matrix3 {
    fn from(q: Quaternion) -> Self {
        Self {
            internal: cgmath::Matrix3::from(q.internal),
        }
    }
}

impl Matrix3 {
    pub fn invert(&self) -> Result<Self, ()> {
        let internal = self.internal.invert();
        if internal.is_none() {
            return Err(());
        }

        Ok(Self {
            internal: internal.unwrap(),
        })
    }

    pub fn x(&self) -> Vector3 {
        Vector3 {
            internal: self.internal.x,
        }
    }
}

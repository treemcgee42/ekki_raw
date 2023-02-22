use std::ops::{Add, Neg};

use cgmath::InnerSpace;

use super::point::Point3;
use super::Float;

#[derive(Clone)]
pub struct Vector2 {
    pub(super) internal: cgmath::Vector2<Float>,
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            internal: -self.internal,
        }
    }
}

impl Into<[f32; 2]> for Vector2 {
    fn into(self) -> [f32; 2] {
        self.internal.into()
    }
}

impl From<eframe::egui::Vec2> for Vector2 {
    fn from(v: eframe::egui::Vec2) -> Self {
        Self::new(v.x as Float, v.y as Float)
    }
}

impl Vector2 {
    pub fn new(x: Float, y: Float) -> Self {
        Self {
            internal: cgmath::Vector2::new(x, y),
        }
    }

    pub fn x(&self) -> Float {
        self.internal.x
    }

    pub fn y(&self) -> Float {
        self.internal.y
    }

    pub fn normalize(self) -> Self {
        Self {
            internal: self.internal.normalize(),
        }
    }

    /// Get the vector perpindicular to the parameter, which the direction you would
    /// get by rotating the parameter clockwise. The length of the resulting vector
    /// is the same as the parameter.
    pub fn get_perpindicular_cw_vector(v: Self) -> Self {
        Self {
            internal: cgmath::Vector2::new(v.y(), -v.x()),
        }
    }

    pub fn are_approximately_equal(v1: &Self, v2: &Self) -> bool {
        if Float::abs(v1.x() - v2.x()) < Float::EPSILON
            && Float::abs(v1.y() - v2.y()) < Float::EPSILON
        {
            return true;
        }

        false
    }
}

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
    pub fn x(&self) -> Float {
        self.internal.x
    }

    pub fn y(&self) -> Float {
        self.internal.y
    }

    pub fn xy(&self) -> Vector2 {
        Vector2 {
            internal: cgmath::Vector2::new(self.x(), self.y()),
        }
    }

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

use super::Float;

#[derive(Clone, Copy)]
pub struct Point3 {
    pub(super) internal: cgmath::Point3<Float>,
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
}

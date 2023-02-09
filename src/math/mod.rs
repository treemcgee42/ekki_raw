pub mod matrix;
pub mod point;
pub mod quaternion;
pub mod vector;

pub type Float = f32;

pub struct Radians(pub Float);

impl Radians {
    fn to_cgmath_radians(&self) -> cgmath::Rad<Float> {
        cgmath::Rad(self.0)
    }
}

pub struct Degrees(pub Float);

impl Degrees {
    fn to_cgmath_degrees(&self) -> cgmath::Deg<Float> {
        cgmath::Deg(self.0)
    }
}

impl Into<Radians> for Degrees {
    fn into(self) -> Radians {
        let rad: cgmath::Rad<Float> = cgmath::Deg(self.0).into();
        Radians(rad.0)
    }
}

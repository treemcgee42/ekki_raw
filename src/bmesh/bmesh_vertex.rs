use std::{cell::RefCell, rc::Rc};

use crate::math::point::Point3;

pub struct BMeshVertex {
    id: u32,
    position: Point3,
}

impl BMeshVertex {
    pub fn create_from_position(position: Point3, rng: &mut impl rand::Rng) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            id: rng.next_u32(),
            position,
        }))
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

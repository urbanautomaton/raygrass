use crate::geometry::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec,
    pub direction: Vec,
}

impl Ray {
    pub fn new(origin: Vec, direction: Vec) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn at(&self, t: f64) -> Vec {
        self.origin + self.direction * t
    }
}

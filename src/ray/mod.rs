use crate::vector::Vec;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec,
    pub direction: Vec,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec {
        self.origin.add(self.direction.scale(t))
    }
}

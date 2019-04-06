use crate::vector::Vec;

pub struct Film {
    top_left: Vec,
    width: f64,
    height: f64,
    z: f64,
}

impl Film {
    pub fn new(top_left: Vec, bottom_right: Vec) -> Film {
        Film {
            top_left,
            width: bottom_right.x - top_left.x,
            height: top_left.y - bottom_right.y,
            z: top_left.z,
        }
    }

    pub fn project(&self, x: f64, y: f64) -> Vec {
        Vec::new(
            self.top_left.x + (x * self.width),
            self.top_left.y + (y * self.height),
            self.z
        )
    }
}


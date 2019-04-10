use crate::vector::Vec;
use crate::film::Film;
use crate::ray::Ray;

pub struct Camera {
    pub eye: Vec,
    pub film: Film,
}

impl Camera {
    pub fn trace(&self, x: f64, y: f64) -> Ray {
        let direction = self.film.project(x, y).subtract(self.eye).normalize();

        Ray {
            origin: self.eye,
            direction,
        }
    }
}


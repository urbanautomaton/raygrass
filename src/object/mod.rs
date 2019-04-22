pub mod sphere;
pub mod plane;

use crate::vector::Vec;
use crate::ray::Ray;
use crate::color::Color;

pub trait Object {
    fn intersect(&self, ray: Ray) -> Option<f64>;
    fn surface_normal(&self, point: Vec) -> Vec;
    fn color_at(&self, point: Vec) -> Color;
    fn reflectance(&self) -> f64;
}

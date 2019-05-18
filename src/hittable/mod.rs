use crate::vector::Vec;
use crate::ray::Ray;
use crate::color::Color;

#[derive(Debug, PartialEq)]
pub struct Hit {
    pub t: f64,
    pub p: Vec,
    pub normal: Vec,
    pub color: Color,
    pub reflectance: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

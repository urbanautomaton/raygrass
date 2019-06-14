use crate::color::Color;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec;

pub struct Hit<'a> {
    pub t: f64,
    pub p: Vec,
    pub normal: Vec,
    pub color: Color,
    pub reflectance: f64,
    pub material: &'a (Material + Send + Sync),
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
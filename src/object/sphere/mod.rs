use crate::vector::Vec;
use crate::ray::Ray;
use crate::color::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    center: Vec,
    radius: f64,
    pub color: Color,
    pub reflectance: f64,
}

impl Sphere {
    pub fn new(center: Vec, radius: f64, color: Color, reflectance: f64) -> Self {
        Self { center, radius, color, reflectance }
    }

    pub fn intersect(&self, ray: Ray) -> Option<f64> {
        let oc = ray.origin.subtract(self.center);
        let dot = ray.direction.normalize().dot(oc);

        let a = dot.powi(2);
        let b = oc.length().powi(2) - self.radius.powi(2);

        if a < b { return None; }

        let sqrt = (a - b).sqrt();
        let ts = vec![-dot - sqrt, -dot + sqrt];

        let positive_ts: std::vec::Vec<f64> = ts
            .into_iter()
            .filter(|t| *t >= 0.0)
            .collect();

        if positive_ts.len() > 0 {
            Some(positive_ts[0])
        } else {
            None
        }
    }

    pub fn surface_normal(&self, point: Vec) -> Vec {
        point.subtract(self.center).normalize()
    }
}

#[cfg(test)]
mod test;

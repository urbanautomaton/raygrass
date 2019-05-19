use crate::hittable::*;
use crate::vector::Vec;
use crate::ray::Ray;
use crate::color::Color;

#[derive(Debug, PartialEq)]
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

    fn surface_normal(&self, point: Vec) -> Vec {
        (point - self.center).normalize()
    }

    fn color_at(&self, _point: Vec) -> Color {
        self.color
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let dot = ray.direction.normalize().dot(oc);

        let a = dot.powi(2);
        let b = oc.length().powi(2) - self.radius.powi(2);

        if a < b { return None; }

        let sqrt = (a - b).sqrt();
        let ts = vec![-dot - sqrt, -dot + sqrt];

        let valid_ts: std::vec::Vec<f64> = ts
            .into_iter()
            .filter(|t| *t >= t_min && *t <= t_max)
            .collect();

        if valid_ts.len() > 0 {
            let t = valid_ts[0];
            let p = ray.at(t);
            let normal = self.surface_normal(p);
            let color = self.color_at(p);

            Some(Hit { t, p, normal, color, reflectance: self.reflectance })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test;

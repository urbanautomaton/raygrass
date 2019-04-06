use crate::vector::Vec;
use crate::object::sphere::Sphere;
use crate::ray::Ray;

pub struct Light {
    center: Vec,
    power: f64,
}

impl Light {
    pub fn new(center: Vec, power: f64) -> Light {
        Light { center, power }
    }

    pub fn illuminate(&self, point: Vec, normal: Vec, objects: &std::vec::Vec<Sphere>) -> f64 {
        let point_to_light = self.center.subtract(point);
        let length = point_to_light.length();

        let shadow_ray = Ray {
            origin: point,
            direction: point_to_light.normalize()
        };

        let occluded = objects.into_iter().any(|s|
            match s.intersect(shadow_ray) {
                Some(t) => t > 1e-10 && t < length,
                _ => false
            }
        );

        if occluded { return 0.0; }

        let cosine = point_to_light.dot(normal) / length;
        let numerator = self.power * cosine;
        let denominator = 4.0 * std::f64::consts::PI * length.powi(2);

        let power = numerator / denominator;

        if power > 0.0 { power } else { 0.0 }
    }
}

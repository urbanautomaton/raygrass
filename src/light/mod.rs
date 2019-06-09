use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vector::Vec;

pub struct Light {
    center: Vec,
    power: f64,
}

impl Light {
    pub fn new(center: Vec, power: f64) -> Light {
        Light { center, power }
    }

    pub fn illuminate(
        &self,
        point: Vec,
        normal: Vec,
        objects: &[Box<Hittable + Sync + Send>],
    ) -> f64 {
        let point_to_light = self.center - point;
        let length = point_to_light.length();

        let shadow_ray = Ray {
            origin: point,
            direction: point_to_light.normalize(),
        };

        let occluded = objects
            .iter()
            .any(|s| s.hit(&shadow_ray, 1e-10, length).is_some());

        if occluded {
            return 0.0;
        }

        let cosine = point_to_light.dot(normal) / length;
        let numerator = self.power * cosine;
        let denominator = 4.0 * std::f64::consts::PI * length.powi(2);

        let power = numerator / denominator;

        if power > 0.0 {
            power
        } else {
            0.0
        }
    }
}

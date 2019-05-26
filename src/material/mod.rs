use rand::prelude::*;
use crate::vector::Vec;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Ray;
}

pub struct ReflectiveMaterial { }

impl Material for ReflectiveMaterial {
    fn scatter(&self, ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Ray {
        let dot = ray_in.direction.dot(*normal);
        let reflection_direction = ray_in.direction - *normal * (2.0 * dot);

        Ray { origin: *intersection, direction: reflection_direction.normalize() }
    }
}

pub struct FuzzyReflectiveMaterial {
    pub fuzz: f64,
}

impl Material for FuzzyReflectiveMaterial {
    fn scatter(&self, ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Ray {
        let dot = ray_in.direction.dot(*normal);
        let reflection_direction = ray_in.direction - *normal * (2.0 * dot);
        let fuzz_vector = Vec::new(random::<f64>(), random::<f64>(), random::<f64>()) * self.fuzz;

        Ray { origin: *intersection, direction: (reflection_direction + fuzz_vector).normalize() }
    }
}

pub struct LambertianMaterial { }

impl LambertianMaterial {
    fn random_in_unit_sphere() -> Vec {
        let mut vec;

        loop {
            vec = Vec::new(random::<f64>(), random::<f64>(), random::<f64>());

            if vec.length() <= 1.0 {
                break vec
            }
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Ray {
        let direction = (Self::random_in_unit_sphere() + normal.normalize()).normalize();
        let origin = *intersection;

        Ray { origin, direction }
    }
}

pub struct DielectricMaterial {
    pub refractive_index: f64,
}

impl DielectricMaterial {
    fn refract(direction: &Vec, normal: &Vec, ni_over_nt: f64) -> Option<Vec> {
        let uv = direction.normalize();
        let dt = uv.dot(*normal);
        let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));

        if discriminant > 0.0 {
            let refracted = (uv - *normal * dt) * ni_over_nt - *normal * discriminant.sqrt();

            Some(refracted)
        } else {
            None
        }
    }

    fn reflect(ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Ray {
        let dot = ray_in.direction.dot(*normal);
        let direction = (ray_in.direction - *normal * (2.0 * dot)).normalize();
        let origin = *intersection;

        Ray { origin, direction }
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Ray {
        let outward_normal;
        let ni_over_nt;

        if ray_in.direction.dot(*normal) > 0.0 {
            outward_normal = *normal * -1.0;
            ni_over_nt = self.refractive_index;
        } else {
            outward_normal = *normal;
            ni_over_nt = 1.0 / self.refractive_index;
        }

        if let Some(refracted) = Self::refract(&ray_in.direction, &outward_normal, ni_over_nt) {
            Ray { origin: *intersection, direction: refracted.normalize() }
        } else {
            Self::reflect(ray_in, intersection, normal)
        }
    }
}

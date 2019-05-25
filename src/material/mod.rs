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
        let reflection_point = *intersection + *normal * 1e-10;

        Ray { origin: reflection_point, direction: reflection_direction.normalize() }
    }
}

pub struct FuzzyReflectiveMaterial {
    pub fuzz: f64,
}

impl Material for FuzzyReflectiveMaterial {
    fn scatter(&self, ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Ray {
        let dot = ray_in.direction.dot(*normal);
        let reflection_direction = ray_in.direction - *normal * (2.0 * dot);
        let reflection_point = *intersection + *normal * 1e-10;
        let fuzz_vector = Vec::new(random::<f64>(), random::<f64>(), random::<f64>()) * self.fuzz;

        Ray { origin: reflection_point, direction: (reflection_direction + fuzz_vector).normalize() }
    }
}

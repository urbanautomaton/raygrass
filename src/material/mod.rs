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

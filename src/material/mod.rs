use crate::ray::Ray;
use crate::vector::Vec;
use rand::prelude::*;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Option<Ray>;
}

pub struct ReflectiveMaterial {}

impl Material for ReflectiveMaterial {
    fn scatter(&self, ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Option<Ray> {
        let dot = ray_in.direction.dot(*normal);
        let reflection_direction = ray_in.direction - *normal * (2.0 * dot);

        Some(Ray {
            origin: *intersection,
            direction: reflection_direction.normalize(),
        })
    }
}

pub struct FuzzyReflectiveMaterial {
    pub fuzz: f64,
}

impl Material for FuzzyReflectiveMaterial {
    fn scatter(&self, ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Option<Ray> {
        let dot = ray_in.direction.dot(*normal);
        let reflection_direction = ray_in.direction - *normal * (2.0 * dot);

        let fuzz_vector = Vec::new(random::<f64>(), random::<f64>(), random::<f64>()) * self.fuzz;
        let scattered = reflection_direction + fuzz_vector;

        if scattered.dot(*normal) > 0.0 {
            Some(Ray {
                origin: *intersection,
                direction: scattered.normalize(),
            })
        } else {
            None
        }
    }
}

pub struct LambertianMaterial {}

impl LambertianMaterial {
    fn random_in_unit_sphere() -> Vec {
        let mut vec;

        loop {
            vec = Vec::new(random::<f64>(), random::<f64>(), random::<f64>());

            if vec.length() <= 1.0 {
                break vec;
            }
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Option<Ray> {
        let direction = (Self::random_in_unit_sphere() + normal.normalize()).normalize();
        let origin = *intersection;

        Some(Ray { origin, direction })
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

    fn reflect(ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Option<Ray> {
        let dot = ray_in.direction.dot(*normal);
        let direction = (ray_in.direction - *normal * (2.0 * dot)).normalize();
        let origin = *intersection;

        Some(Ray { origin, direction })
    }

    fn schlick(&self, cosine: f64) -> f64 {
        let r0 = ((1.0 - self.refractive_index) / (1.0 + self.refractive_index)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray_in: &Ray, intersection: &Vec, normal: &Vec) -> Option<Ray> {
        let outward_normal;
        let ni_over_nt;
        let cosine;
        let rdotn = ray_in.direction.normalize().dot(*normal);

        if rdotn > 0.0 {
            outward_normal = *normal * -1.0;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * rdotn;
        } else {
            outward_normal = *normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -rdotn;
        }

        let reflect_prob = self.schlick(cosine);

        if let Some(refracted) = Self::refract(&ray_in.direction, &outward_normal, ni_over_nt) {
            if random::<f64>() < reflect_prob {
                Self::reflect(ray_in, intersection, normal)
            } else {
                Some(Ray {
                    origin: *intersection,
                    direction: refracted.normalize(),
                })
            }
        } else {
            Self::reflect(ray_in, intersection, normal)
        }
    }
}

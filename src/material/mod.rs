use rand::Rng;
use rand_xoshiro::Xoshiro256StarStar;

use crate::hittable::Hit;
use crate::ray::Ray;
use crate::vector::Vec;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Xoshiro256StarStar) -> Option<Ray>;
}

pub struct ReflectiveMaterial {}

impl Material for ReflectiveMaterial {
    fn scatter(&self, ray: &Ray, hit: &Hit, _rng: &mut Xoshiro256StarStar) -> Option<Ray> {
        let dot = ray.direction.dot(hit.normal);
        let reflection_direction = ray.direction - hit.normal * (2.0 * dot);

        Some(Ray::new(hit.p, reflection_direction))
    }
}

pub struct FuzzyReflectiveMaterial {
    pub fuzz: f64,
}

impl Material for FuzzyReflectiveMaterial {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Xoshiro256StarStar) -> Option<Ray> {
        let dot = ray.direction.dot(hit.normal);
        let reflection_direction = ray.direction - hit.normal * (2.0 * dot);

        let coords: [f64; 3] = rng.gen();
        let fuzz_vector = Vec::from(coords) * self.fuzz;
        let scattered = reflection_direction + fuzz_vector;

        if scattered.dot(hit.normal) > 0.0 {
            Some(Ray::new(hit.p, scattered.normalize()))
        } else {
            None
        }
    }
}

pub struct LambertianMaterial {}

impl LambertianMaterial {
    fn random_in_unit_sphere(rng: &mut Xoshiro256StarStar) -> Vec {
        let mut vec;

        loop {
            let coords: [f64; 3] = rng.gen();

            vec = Vec::from(coords);

            if vec.length() <= 1.0 {
                break vec;
            }
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray: &Ray, hit: &Hit, rng: &mut Xoshiro256StarStar) -> Option<Ray> {
        let direction = Self::random_in_unit_sphere(rng) + hit.normal;

        Some(Ray::new(hit.p, direction))
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

    fn reflect(ray: &Ray, hit: &Hit) -> Option<Ray> {
        let dot = ray.direction.dot(hit.normal);
        let direction = ray.direction - hit.normal * (2.0 * dot);

        Some(Ray::new(hit.p, direction))
    }

    fn schlick(&self, cosine: f64) -> f64 {
        let r0 = ((1.0 - self.refractive_index) / (1.0 + self.refractive_index)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Xoshiro256StarStar) -> Option<Ray> {
        let outward_normal;
        let ni_over_nt;
        let cosine;
        let rdotn = ray.direction.dot(hit.normal);

        if rdotn > 0.0 {
            outward_normal = hit.normal * -1.0;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * rdotn;
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -rdotn;
        }

        let reflect_prob = self.schlick(cosine);

        if let Some(refracted) = Self::refract(&ray.direction, &outward_normal, ni_over_nt) {
            let val: f64 = rng.gen();

            if val < reflect_prob {
                Self::reflect(ray, hit)
            } else {
                Some(Ray::new(hit.p, refracted))
            }
        } else {
            Self::reflect(ray, hit)
        }
    }
}

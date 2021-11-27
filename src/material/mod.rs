use rand::Rng;
use rand_xoshiro::Xoshiro256StarStar;

use crate::color::Color;
use crate::geometry::*;
use crate::hittable::Hit;
use crate::ray::Ray;
use crate::texture::Texture;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Xoshiro256StarStar) -> Option<(Ray, Color)> {
        self.scatter_ray(ray, hit, rng)
            .map(|scattered| (scattered, self.attenuation(hit)))
    }

    fn scatter_ray(&self, ray: &Ray, hit: &Hit, rng: &mut Xoshiro256StarStar) -> Option<Ray>;
    fn attenuation(&self, hit: &Hit) -> Color;
}

pub struct ReflectiveMaterial<T: Texture> {
    pub texture: T,
}

impl<T: Texture> Material for ReflectiveMaterial<T> {
    fn scatter_ray(&self, ray: &Ray, hit: &Hit, _rng: &mut Xoshiro256StarStar) -> Option<Ray> {
        let reflection_direction = ray.direction.reflect(hit.normal);

        Some(Ray::new(hit.p, reflection_direction))
    }

    fn attenuation(&self, hit: &Hit) -> Color {
        self.texture.color(hit.u, hit.v, &hit.p)
    }
}

pub struct FuzzyReflectiveMaterial<T: Texture> {
    pub texture: T,
    pub fuzz: f64,
}

impl<T: Texture> Material for FuzzyReflectiveMaterial<T> {
    fn scatter_ray(&self, ray: &Ray, hit: &Hit, rng: &mut Xoshiro256StarStar) -> Option<Ray> {
        let reflection_direction = ray.direction.reflect(hit.normal);

        let coords: [f64; 3] = rng.gen();
        let fuzz_vector = Vector3::from(coords) * self.fuzz;
        let scattered: Unit3 = (Vector3::from(reflection_direction) + fuzz_vector).into();

        if scattered.dot(hit.normal) > 0.0 {
            Some(Ray::new(hit.p, scattered))
        } else {
            None
        }
    }

    fn attenuation(&self, hit: &Hit) -> Color {
        self.texture.color(hit.u, hit.v, &hit.p)
    }
}

pub struct LambertianMaterial<T: Texture> {
    pub texture: T,
}

impl<T: Texture> LambertianMaterial<T> {
    fn random_in_unit_sphere(rng: &mut Xoshiro256StarStar) -> Vector3 {
        let mut vec;

        loop {
            let coords: [f64; 3] = rng.gen();

            vec = Vector3::from(coords);

            if vec.length() <= 1.0 {
                break vec;
            }
        }
    }
}

impl<T: Texture> Material for LambertianMaterial<T> {
    fn scatter_ray(&self, _ray: &Ray, hit: &Hit, rng: &mut Xoshiro256StarStar) -> Option<Ray> {
        let direction = (Self::random_in_unit_sphere(rng) + Vector3::from(hit.normal)).into();

        Some(Ray::new(hit.p, direction))
    }

    fn attenuation(&self, hit: &Hit) -> Color {
        self.texture.color(hit.u, hit.v, &hit.p)
    }
}

pub struct DielectricMaterial<T: Texture> {
    pub texture: T,
    pub refractive_index: f64,
}

impl<T: Texture> DielectricMaterial<T> {
    fn refract(direction: &Unit3, normal: &Unit3, ni_over_nt: f64) -> Option<Unit3> {
        let uv: Vector3 = (*direction).into();
        let un: Vector3 = (*normal).into();
        let dt = uv.dot(un);
        let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));

        if discriminant > 0.0 {
            let refracted = (uv - *normal * dt) * ni_over_nt - *normal * discriminant.sqrt();

            Some(refracted.into())
        } else {
            None
        }
    }

    fn reflect(ray: &Ray, hit: &Hit) -> Option<Ray> {
        let direction = ray.direction.reflect(hit.normal);

        Some(Ray::new(hit.p, direction))
    }

    fn schlick(&self, cosine: f64) -> f64 {
        let r0 = ((1.0 - self.refractive_index) / (1.0 + self.refractive_index)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl<T: Texture> Material for DielectricMaterial<T> {
    fn scatter_ray(&self, ray: &Ray, hit: &Hit, rng: &mut Xoshiro256StarStar) -> Option<Ray> {
        let outward_normal;
        let ni_over_nt;
        let cosine;
        let rdotn = ray.direction.dot(hit.normal);

        if rdotn > 0.0 {
            outward_normal = hit.normal.reverse();
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

    fn attenuation(&self, hit: &Hit) -> Color {
        self.texture.color(hit.u, hit.v, &hit.p)
    }
}

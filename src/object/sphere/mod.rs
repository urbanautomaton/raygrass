use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec;

pub struct Sphere<M: Material> {
    center: Vec,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    fn surface_normal(&self, point: Vec) -> Vec {
        (point - self.center).normalize()
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let dot = ray.direction.dot(oc);

        let a = dot.powi(2);
        let b = oc.length().powi(2) - self.radius.powi(2);

        if a < b {
            return None;
        }

        let sqrt = (a - b).sqrt();

        for t in &[-dot - sqrt, -dot + sqrt] {
            if (t_min..t_max).contains(t) {
                let p = ray.at(*t);
                let normal = self.surface_normal(p);

                return Some(Hit {
                    t: *t,
                    p,
                    u: 0.,
                    v: 0.,
                    normal,
                    material: &self.material,
                });
            }
        }

        None
    }
}

impl<M: Material> Bounded for Sphere<M> {
    fn bounding_box(&self) -> BoundingBox {
        let offset = Vec::new(self.radius, self.radius, self.radius);

        BoundingBox {
            min: self.center - offset,
            max: self.center + offset,
        }
    }
}

#[cfg(test)]
mod test;

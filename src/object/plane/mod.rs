use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec;

pub struct Plane<M: Material> {
    point: Vec,
    normal: Vec,
    material: M,
}

impl<M: Material> Plane<M> {
    pub fn new(point: Vec, normal: Vec, material: M) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            material,
        }
    }
}

impl<M: Material> Hittable for Plane<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let ndotl = self.normal.dot(ray.direction);

        if ndotl.abs() < 1e-10 {
            None
        } else {
            let t = self.normal.dot(self.point - ray.origin) / ndotl;
            let p = ray.at(t);

            if t < t_min || t > t_max {
                None
            } else {
                Some(Hit {
                    t,
                    p,
                    u: 0.,
                    v: 0.,
                    normal: self.normal,
                    material: &self.material,
                })
            }
        }
    }
}

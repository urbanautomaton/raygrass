use crate::geometry::*;
use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;

pub struct Plane<M: Material> {
    point: Point3,
    u: Vector3,
    v: Vector3,
    normal: Vector3,
    material: M,
}

impl<M: Material> Plane<M> {
    pub fn new(point: Point3, u: Vector3, v: Vector3, material: M) -> Self {
        let normal = (u * v).normalize();

        Self {
            point,
            u,
            v,
            normal,
            material,
        }
    }

    pub fn uv(&self, point: Point3) -> (f64, f64) {
        let pv = point - self.point;

        (pv.dot(self.u), pv.dot(self.v))
    }
}

impl<M: Material> Hittable for Plane<M> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let ndotl = self.normal.dot(ray.direction);

        if ndotl.abs() < 1e-10 {
            None
        } else {
            let t = self.normal.dot(self.point - ray.origin) / ndotl;

            if t < t_min || t > t_max {
                None
            } else {
                let p = ray.at(t);
                let uv = self.uv(p);

                Some(Hit {
                    t,
                    p,
                    u: uv.0,
                    v: uv.1,
                    normal: self.normal,
                    material: &self.material,
                })
            }
        }
    }
}

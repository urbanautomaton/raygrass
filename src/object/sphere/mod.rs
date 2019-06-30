use crate::color::Color;
use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vec;

pub struct Sphere<'a> {
    center: Vec,
    radius: f64,
    pub color: Color,
    pub reflectance: f64,
    pub material: &'a (Material + Send + Sync),
}

impl<'a> Sphere<'a> {
    pub fn new(
        center: Vec,
        radius: f64,
        color: Color,
        reflectance: f64,
        material: &'a (Material + Send + Sync),
    ) -> Self {
        Self {
            center,
            radius,
            color,
            reflectance,
            material,
        }
    }

    fn surface_normal(&self, point: Vec) -> Vec {
        (point - self.center).normalize()
    }

    fn color_at(&self, _point: Vec) -> Color {
        self.color
    }
}

impl<'a> Hittable for Sphere<'a> {
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
                let color = self.color_at(p);

                return Some(Hit {
                    t: *t,
                    p,
                    normal,
                    color,
                    reflectance: self.reflectance,
                    material: self.material,
                });
            }
        }

        return None;
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        let offset = Vec::new(self.radius, self.radius, self.radius);

        Some(BoundingBox {
            min: self.center - offset,
            max: self.center + offset,
        })
    }
}

#[cfg(test)]
mod test;

use crate::hittable::*;
use crate::vector::Vec;
use crate::ray::Ray;
use crate::color::Color;

#[derive(Debug)]
pub struct Plane {
    point: Vec,
    normal: Vec,
    pub color: Color,
    pub reflectance: f64,
}

impl Plane {
    pub fn new(point: Vec, normal: Vec, color: Color, reflectance: f64) -> Self {
        Self { point, normal, color, reflectance }
    }

    fn color_at(&self, point: Vec) -> Color {
        if (point.x.round() + point.z.round()).abs() % 2.0 < 1e-10 {
            Color::new(10.0, 10.0, 10.0)
        } else {
            self.color
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let ndotl = self.normal.dot(ray.direction);

        if ndotl.abs() < 1e-10 { 
            None
        } else {
            let t = self.normal.dot(self.point.subtract(ray.origin)) / ndotl;
            let p = ray.at(t);
            let color = self.color_at(p);

            if t < t_min || t > t_max {
                None
            } else {
                Some(Hit {
                    t,
                    p,
                    normal: self.normal,
                    color,
                    reflectance: self.reflectance
                })
            }
        }
    }
}

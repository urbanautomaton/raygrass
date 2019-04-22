use crate::object::Object;
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
}

impl Object for Plane {
    fn intersect(&self, ray: Ray) -> Option<f64> {
        let ndotl = self.normal.dot(ray.direction);

        if ndotl.abs() < 1e-10 { 
            None
        } else {
            let t = self.normal.dot(self.point.subtract(ray.origin)) / ndotl;
            if t < 0.0 { None } else { Some(t) }
        }
    }

    fn surface_normal(&self, _point: Vec) -> Vec {
        self.normal
    }

    fn color_at(&self, point: Vec) -> Color {
        if (point.x.round() + point.z.round()).abs() % 2.0 < 1e-10 {
            Color::new(10.0, 10.0, 10.0)
        } else {
            self.color
        }
    }

    fn reflectance(&self) -> f64 {
        self.reflectance
    }
}

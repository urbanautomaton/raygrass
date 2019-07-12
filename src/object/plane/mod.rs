use crate::color::Color;
use crate::hittable::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vector::Vec;

pub struct Plane<'a, T: Texture> {
    point: Vec,
    normal: Vec,
    texture: T,
    pub reflectance: f64,
    pub material: &'a Material,
}

impl<'a, T: Texture> Plane<'a, T> {
    pub fn new(
        point: Vec,
        normal: Vec,
        texture: T,
        reflectance: f64,
        material: &'a Material,
    ) -> Self {
        Self {
            point,
            normal: normal.normalize(),
            texture,
            reflectance,
            material,
        }
    }

    fn color_at(&self, point: Vec) -> Color {
        if (point.x.round() + point.z.round()).abs() % 2.0 < 1e-10 {
            Color::new(10.0, 10.0, 10.0)
        } else {
            self.texture.color(0., 0.)
        }
    }
}

impl<'a, T: Texture> Hittable for Plane<'a, T> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let ndotl = self.normal.dot(ray.direction);

        if ndotl.abs() < 1e-10 {
            None
        } else {
            let t = self.normal.dot(self.point - ray.origin) / ndotl;
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
                    reflectance: self.reflectance,
                    material: self.material,
                })
            }
        }
    }
}

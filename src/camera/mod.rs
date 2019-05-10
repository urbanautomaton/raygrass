extern crate image;

use crate::vector::Vec;
use crate::color::Color;
use crate::object::Object;
use crate::light::Light;
use crate::film::Film;
use crate::ray::Ray;

pub struct Camera {
    pub eye: Vec,
    pub film: Film,
    pub img_x: u32,
    pub img_y: u32,
}

impl Camera {

    pub fn capture(&self, objects: &[Box<Object>], lights: &[Light], outfile: &str) -> () {
        let mut buf = image::ImageBuffer::new(self.img_x, self.img_y);

        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            let ray = self.ray_for_pixel(x as f64 / self.img_x as f64, y as f64 / self.img_y as f64);

            let color = self.trace(objects, lights, ray, 50)
                .unwrap_or(Color::new(30.0, 30.0, 30.0));

            *pixel = image::Rgb([color.r as u8, color.g as u8, color.b as u8]);
        }

        buf.save(outfile).expect("Saving image failed");
    }

    fn ray_for_pixel(&self, x: f64, y: f64) -> Ray {
        let direction = self.film
            .project(x, y)
            .subtract(self.eye)
            .normalize();

        Ray {
            origin: self.eye,
            direction,
        }
    }

    fn trace(&self, objects: &[Box<Object>], lights: &[Light], ray: Ray, remaining_calls: u32) -> Option<Color> {
        if remaining_calls <= 0 {
            return None;
        }

        let mut min_t = std::f64::INFINITY;
        let mut min_object: Option<&Box<Object>> = None;

        for object in objects {
            if let Some(t) = object.intersect(ray) {
                if t < min_t {
                    min_t = t;
                    min_object = Some(object);
                }
            }
        }

        if let Some(hit) = min_object {
            let intersection = ray.at(min_t);
            let normal = hit.surface_normal(intersection);

            let energy = lights
                .iter()
                .fold(0.0, |acc, light|
                      acc + light.illuminate(intersection, normal, &objects)
                );

            let surface_color = hit.color_at(intersection);
            let illuminated_color = surface_color.scale(energy).scale(1.0 - hit.reflectance());

            let dot = ray.direction.dot(normal);
            let reflection_direction = ray.direction.subtract(normal.scale(2.0 * dot));
            let reflection_point = intersection.add(normal.scale(1e-10));
            let reflection_ray = Ray { origin: reflection_point, direction: reflection_direction };

            if let Some(incoming_color) = self.trace(objects, lights, reflection_ray, remaining_calls - 1) {
                let reflection_color = surface_color.scale(hit.reflectance() / 255.0);
                let reflected_color = Color::new(
                    incoming_color.r * reflection_color.r,
                    incoming_color.g * reflection_color.g,
                    incoming_color.b * reflection_color.b,
                    );

                Some(illuminated_color.add(reflected_color))
            } else {
                Some(illuminated_color)
            }
        } else {
            None
        }
    }
}


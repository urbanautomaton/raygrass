extern crate image;

use std::cmp::Ordering;
use rand::prelude::*;

use crate::vector::Vec;
use crate::color::Color;
use crate::hittable::*;
use crate::light::Light;
use crate::film::Film;
use crate::ray::Ray;

pub struct Camera {
    pub eye: Vec,
    pub film: Film,
    pub img_x: u32,
    pub img_y: u32,
    pub samples: u32,
}

impl Camera {

    pub fn capture(&self, objects: &[Box<Hittable>], lights: &[Light], outfile: &str) -> () {
        let mut buf = image::ImageBuffer::new(self.img_x, self.img_y);

        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            let mut r: f64 = 0.0;
            let mut g: f64 = 0.0;
            let mut b: f64 = 0.0;

            for _ in 0..self.samples {
                let ray = self.ray_for_pixel(x, y);

                let color = self.trace(objects, lights, ray, 50)
                    .unwrap_or(Color::new(30.0, 30.0, 30.0));

                r += color.r;
                g += color.g;
                b += color.b;
            }


            *pixel = image::Rgb([(r / self.samples as f64) as u8, (g / self.samples as f64) as u8, (b / self.samples as f64) as u8]);
        }

        buf.save(outfile).expect("Saving image failed");
    }

    fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        let direction = self.film
            .project(
                x as f64 / self.img_x as f64 + random::<f64>() / self.img_x as f64,
                y as f64 / self.img_y as f64 + random::<f64>() / self.img_y as f64,
                )
            .subtract(self.eye)
            .normalize();

        Ray { origin: self.eye, direction }
    }

    fn ray_hit(&self, objects: &[Box<Hittable>], ray: Ray) -> Option<Hit> {
        objects.iter()
            .filter_map(|o| o.hit(&ray, 0.0, std::f64::INFINITY))
            .min_by(|h1, h2| h1.t.partial_cmp(&h2.t).unwrap_or(Ordering::Equal))
    }

    fn trace(&self, objects: &[Box<Hittable>], lights: &[Light], ray: Ray, remaining_calls: u32) -> Option<Color> {
        if remaining_calls <= 0 {
            return None;
        }

        if let Some(hit) = self.ray_hit(objects, ray) {
            let intersection = hit.p;
            let normal = hit.normal;

            let energy = lights
                .iter()
                .fold(0.0, |acc, light|
                      acc + light.illuminate(intersection, normal, &objects)
                );

            let surface_color = hit.color;
            let illuminated_color = surface_color.scale(energy).scale(1.0 - hit.reflectance);

            let dot = ray.direction.dot(normal);
            let reflection_direction = ray.direction.subtract(normal.scale(2.0 * dot));
            let reflection_point = intersection.add(normal.scale(1e-10));
            let reflection_ray = Ray { origin: reflection_point, direction: reflection_direction };

            if let Some(incoming_color) = self.trace(objects, lights, reflection_ray, remaining_calls - 1) {
                let reflection_color = surface_color.scale(hit.reflectance / 255.0);
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


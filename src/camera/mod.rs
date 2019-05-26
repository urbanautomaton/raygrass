extern crate image;
extern crate indicatif;

use std::cmp::Ordering;
use rand::prelude::*;

use indicatif::{ProgressBar, ProgressStyle};

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

    pub fn capture(&self, objects: &[Box<Hittable>], lights: &[Light], outfile: &str) {
        let mut buf = image::ImageBuffer::new(self.img_x, self.img_y);
        let pb = ProgressBar::new((self.img_x * self.img_y).into());
        pb.set_style(ProgressStyle::default_bar()
                     .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})")
                     .progress_chars("#>-"));

        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            pb.set_position((x + y * x).into());

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


            *pixel = image::Rgb([(r / f64::from(self.samples)) as u8, (g / f64::from(self.samples)) as u8, (b / f64::from(self.samples)) as u8]);
        }

        buf.save(outfile).expect("Saving image failed");
    }

    fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        let x_frac = f64::from(x) / f64::from(self.img_x) + random::<f64>() / f64::from(self.img_x);
        let y_frac = f64::from(y) / f64::from(self.img_y) + random::<f64>() / f64::from(self.img_y);

        let direction = (self.film.project(x_frac, y_frac) - self.eye).normalize();

        Ray { origin: self.eye, direction }
    }

    fn ray_hit<'a>(&'a self, objects: &'a [Box<Hittable>], ray: Ray) -> Option<Hit> {
        objects.iter()
            .filter_map(|o| o.hit(&ray, 1e-10, std::f64::INFINITY))
            .min_by(|h1, h2| h1.t.partial_cmp(&h2.t).unwrap_or(Ordering::Equal))
    }

    fn ray_color(ray: &Ray) -> Color {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);

        Color::new(
            1.0 - 0.5 * t,
            1.0 - 0.3 * t,
            1.0,
        ).scale(255.0)
    }

    fn trace(&self, objects: &[Box<Hittable>], lights: &[Light], ray: Ray, remaining_calls: u32) -> Option<Color> {
        if remaining_calls == 0 {
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

            let reflection_ray = hit.material.scatter(&ray, &intersection, &normal);

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
            Some(Self::ray_color(&ray))
        }
    }
}


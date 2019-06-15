extern crate image;
extern crate indicatif;
extern crate rayon;

use rand::prelude::*;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

use indicatif::{ProgressBar, ProgressStyle};

use crate::color::Color;
use crate::film::Film;
use crate::hittable::*;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vec;

pub struct Camera {
    pub eye: Vec,
    pub film: Film,
    pub img_x: u32,
    pub img_y: u32,
    pub samples: u32,
}

impl Camera {
    pub fn capture(&self, scene: &Scene, outfile: &str) {
        let buf = Arc::new(Mutex::new(image::ImageBuffer::new(self.img_x, self.img_y)));
        let pb = ProgressBar::new((self.img_x * self.img_y).into());
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})",
                )
                .progress_chars("#>-"),
        );
        pb.set_draw_delta((self.img_x * self.img_y / (100 * self.samples)).into());

        (0..(self.img_x * self.img_y))
            .into_par_iter()
            .for_each(|px| {
                let x = px % self.img_x;
                let y = px / self.img_x;

                pb.inc(1);

                let mut color_acc = Color::new(0., 0., 0.);

                for _ in 0..self.samples {
                    let ray = self.ray_for_pixel(x, y);

                    let color = self
                        .trace(scene, ray, 50)
                        .unwrap_or_else(|| Color::new(30.0, 30.0, 30.0));

                    color_acc = color_acc.add(color);
                }

                color_acc = color_acc.scale(1. / f64::from(self.samples));

                buf.lock()
                    .unwrap()
                    .put_pixel(x, y, image::Rgb(color_acc.into()));
            });

        buf.lock()
            .unwrap()
            .save(outfile)
            .expect("Saving image failed");
    }

    fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        let x_frac = f64::from(x) / f64::from(self.img_x) + random::<f64>() / f64::from(self.img_x);
        let y_frac = f64::from(y) / f64::from(self.img_y) + random::<f64>() / f64::from(self.img_y);

        let direction = self.film.project(x_frac, y_frac) - self.eye;

        Ray::new(self.eye, direction)
    }

    fn ray_hit<'a>(&'a self, objects: &'a [Box<Hittable + Sync + Send>], ray: Ray) -> Option<Hit> {
        objects
            .iter()
            .filter_map(|o| o.hit(&ray, 1e-10, std::f64::INFINITY))
            .min_by(|h1, h2| h1.t.partial_cmp(&h2.t).unwrap_or(Ordering::Equal))
    }

    fn ray_color(ray: &Ray) -> Color {
        let t = 0.5 * (ray.direction.y + 1.0);

        Color::new(1.0 - 0.5 * t, 1.0 - 0.3 * t, 1.0).scale(255.0)
    }

    fn trace(&self, scene: &Scene, ray: Ray, remaining_calls: u32) -> Option<Color> {
        if remaining_calls == 0 {
            return None;
        }

        if let Some(hit) = self.ray_hit(&scene.objects, ray) {
            if let Some(reflection_ray) = hit.material.scatter(&ray, &hit.p, &hit.normal) {
                if let Some(incoming_color) = self.trace(scene, reflection_ray, remaining_calls - 1)
                {
                    let reflection_color = hit.color.scale(hit.reflectance / 255.0);
                    let reflected_color = Color::new(
                        incoming_color.r * reflection_color.r,
                        incoming_color.g * reflection_color.g,
                        incoming_color.b * reflection_color.b,
                    );

                    Some(reflected_color)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            Some(Self::ray_color(&ray))
        }
    }
}

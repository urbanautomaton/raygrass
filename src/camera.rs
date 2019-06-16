extern crate image;
extern crate indicatif;
extern crate rayon;

use rand::prelude::*;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use indicatif::{ProgressBar, ProgressStyle};

use crate::color::Color;
use crate::hittable::*;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vector::Vec;

struct Film {
    top_left: Vec,
    u: Vec,
    v: Vec,
    width: f64,
    height: f64,
}

impl Film {
    pub fn project(&self, x: f64, y: f64) -> Vec {
        self.top_left + self.u * x * self.width + self.v * y * self.height
    }
}

pub struct Camera {
    origin: Vec,
    film: Film,
    img_x: u32,
    img_y: u32,
    samples: u32,
}

impl Camera {
    pub fn new(
        look_from: Vec,
        look_at: Vec,
        fov: f64,
        img_x: u32,
        img_y: u32,
        samples: u32,
    ) -> Self {
        let theta = fov * std::f64::consts::PI / 180.;
        let aspect = f64::from(img_x) / f64::from(img_y);
        let height = (theta / 2.).tan() * 2.;
        let width = height * aspect;
        let origin = look_from;
        let w = (look_at - look_from).normalize();
        let u = (Vec::new(0., 1., 0.) * w).normalize();
        let v = (u * w).normalize();
        let top_left = origin - (u * width / 2.) - (v * height / 2.) + w;

        Self {
            origin,
            film: Film {
                top_left,
                u,
                v,
                width,
                height,
            },
            img_x,
            img_y,
            samples,
        }
    }

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

                    let color = self.trace(scene, ray, 50);

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

        let direction = self.film.project(x_frac, y_frac) - self.origin;

        Ray::new(self.origin, direction)
    }

    fn ray_hit<'a>(&'a self, objects: &'a [Box<Hittable + Sync + Send>], ray: Ray) -> Option<Hit> {
        let mut result: Option<Hit> = None;

        for o in objects {
            if let Some(hit) = o.hit(&ray, 1e-10, std::f64::INFINITY) {
                if let Some(min_hit) = result {
                    if hit.t < min_hit.t {
                        result = Some(hit);
                    } else {
                        result = Some(min_hit);
                    }
                } else {
                    result = Some(hit);
                }
            }
        }

        result
    }

    fn ray_color(ray: &Ray) -> Color {
        let t = 0.5 * (ray.direction.y + 1.0);

        Color::new(1.0 - 0.5 * t, 1.0 - 0.3 * t, 1.0).scale(255.0)
    }

    fn trace(&self, scene: &Scene, ray: Ray, remaining_calls: u32) -> Color {
        if remaining_calls == 0 {
            return Color::new(0., 0., 0.);
        }

        if let Some(hit) = self.ray_hit(&scene.objects, ray) {
            match hit.material.scatter(&ray, &hit.p, &hit.normal) {
                Some(reflection_ray) => {
                    let incoming_color = self.trace(scene, reflection_ray, remaining_calls - 1);
                    let reflection_color = hit.color.scale(hit.reflectance / 255.0);
                    let reflected_color = Color::new(
                        incoming_color.r * reflection_color.r,
                        incoming_color.g * reflection_color.g,
                        incoming_color.b * reflection_color.b,
                    );

                    reflected_color
                }
                None => Color::new(0., 0., 0.),
            }
        } else {
            Self::ray_color(&ray)
        }
    }
}

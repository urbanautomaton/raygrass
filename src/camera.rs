extern crate image;
extern crate indicatif;
extern crate rayon;

use rand::prelude::*;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;
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
    aperture: f64,
    u: Vec,
    v: Vec,
}

impl Camera {
    pub fn new(
        look_from: Vec,
        look_at: Vec,
        fov: f64,
        aperture: f64,
        focus_dist: f64,
        img_x: u32,
        img_y: u32,
    ) -> Self {
        let theta = fov * std::f64::consts::PI / 180.;
        let aspect = f64::from(img_x) / f64::from(img_y);
        let height = (theta / 2.).tan() * 2. * focus_dist;
        let width = height * aspect;
        let origin = look_from;
        let w = (look_at - look_from).normalize();
        let u = (Vec::new(0., 1., 0.) * w).normalize();
        let v = (u * w).normalize();
        let top_left = origin - (u * width / 2.) - (v * height / 2.) + w * focus_dist;

        Self {
            origin,
            film: Film {
                top_left,
                u,
                v,
                width,
                height,
            },
            aperture,
            u,
            v,
            img_x,
            img_y,
        }
    }

    pub fn capture(&self, scene: &Scene, samples: u32, outfile: &str) {
        let buf = Arc::new(Mutex::new(image::ImageBuffer::new(self.img_x, self.img_y)));
        let pb = ProgressBar::new((self.img_x * self.img_y).into());
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})",
                )
                .progress_chars("#>-"),
        );
        pb.set_draw_delta((self.img_x * self.img_y / (100 * samples)).into());

        let pixel_width = 1. / f64::from(self.img_x);
        let pixel_height = 1. / f64::from(self.img_y);
        let color_scale = 1. / f64::from(samples);

        (0..(self.img_x * self.img_y))
            .into_par_iter()
            .for_each(|px| {
                let x = px % self.img_x;
                let y = px / self.img_x;

                let mut rng = Xoshiro256StarStar::from_entropy();

                pb.inc(1);

                let mut color_acc = Color::new(0., 0., 0.);

                let x_min = f64::from(x) * pixel_width;
                let y_min = f64::from(y) * pixel_height;

                for _ in 0..samples {
                    let x_max = x_min + pixel_width;
                    let y_max = y_min + pixel_height;

                    let ray = self.ray_for_pixel(&mut rng, (x_min, x_max), (y_min, y_max));

                    let color = self.trace(scene, ray, 50, &mut rng);

                    color_acc = color_acc.add(color);
                }

                color_acc = color_acc.scale(color_scale);

                buf.lock()
                    .unwrap()
                    .put_pixel(x, y, image::Rgb(color_acc.into()));
            });

        buf.lock()
            .unwrap()
            .save(outfile)
            .expect("Saving image failed");
    }

    fn random_in_unit_disc(rng: &mut Xoshiro256StarStar) -> Vec {
        let mut vec;

        loop {
            let (x, y): (f64, f64) = rng.gen();

            vec = Vec::new(x, y, 0.) * 2. - Vec::new(1., 1., 0.);

            if vec.dot(vec) < 1.0 {
                break vec;
            }
        }
    }

    fn ray_for_pixel(
        &self,
        rng: &mut Xoshiro256StarStar,
        x_range: (f64, f64),
        y_range: (f64, f64),
    ) -> Ray {
        let x_pos = rng.gen_range(x_range.0, x_range.1);
        let y_pos = rng.gen_range(y_range.0, y_range.1);

        let random_disc = Self::random_in_unit_disc(rng) * (self.aperture / 2.);
        let offset = self.u * random_disc.x + self.v * random_disc.y;
        let ray_origin = self.origin + offset;

        let direction = self.film.project(x_pos, y_pos) - ray_origin;

        Ray::new(ray_origin, direction)
    }

    fn ray_hit<'a>(&'a self, objects: &'a [Box<Hittable>], ray: Ray) -> Option<Hit> {
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

    fn trace(
        &self,
        scene: &Scene,
        ray: Ray,
        remaining_calls: u32,
        rng: &mut Xoshiro256StarStar,
    ) -> Color {
        if remaining_calls == 0 {
            return Color::new(0., 0., 0.);
        }

        if let Some(hit) = self.ray_hit(&scene.objects, ray) {
            match hit.material.scatter(&ray, &hit, rng) {
                Some(reflection_ray) => {
                    let incoming_color =
                        self.trace(scene, reflection_ray, remaining_calls - 1, rng);
                    let reflection_color = hit.color.scale(1. / 255.0);

                    Color::new(
                        incoming_color.r * reflection_color.r,
                        incoming_color.g * reflection_color.g,
                        incoming_color.b * reflection_color.b,
                    )
                }
                None => Color::new(0., 0., 0.),
            }
        } else {
            Self::ray_color(&ray)
        }
    }
}

mod color;
mod vector;
mod object;
mod ray;
mod light;
mod film;
mod camera;
mod hittable;
mod material;

use std::sync::Arc;

use vector::Vec;
use color::*;
use hittable::*;
use object::sphere::Sphere;
use object::plane::Plane;
use light::Light;
use film::Film;
use camera::Camera;
use material::*;

fn main() {
    let eye = Vec::new(0.0, 0.8, 0.3);
    let film = Film::new(Vec::new(-0.8, 1.5, 1.3), Vec::new(1.2, 0.0, 1.3));
    let camera = Camera { eye, film, img_x: 1600, img_y: 1200, samples: 200 };

    let objects: std::vec::Vec<Box<Hittable + Send + Sync>> = vec![
        Box::new(Sphere::new(Vec::new(-1.0, 1.0, 5.0), 0.8, rgb!(255.0, 255.0,  255.0), 1.0, Arc::new(DielectricMaterial { refractive_index: 1.3 }))),
        Box::new(Sphere::new(Vec::new(1.2, 1.0, 3.0), 0.4, rgb!(255.0, 255.0,  255.0), 1.0, Arc::new(DielectricMaterial { refractive_index: 1.3 }))),
        Box::new(Sphere::new(Vec::new(1.0,  1.0, 5.0), 0.8, rgb!(50.0,  255.0, 100.0), 1.0, Arc::new(FuzzyReflectiveMaterial { fuzz: 0.1 }))),
        Box::new(Sphere::new(Vec::new(2.5,  1.0, 5.0), 0.8, rgb!(50.0,  100.0, 255.0), 1.0, Arc::new(LambertianMaterial {}))),
        Box::new(Sphere::new(Vec::new(-1.0, 2.0, 4.0), 0.2, rgb!(220.0, 220.0, 75.0), 1.0, Arc::new(ReflectiveMaterial {}))),

        Box::new(Plane::new(Vec::new(0.0, -1.0, 0.0), Vec::new(0.0, 1.0, 0.0), rgb!(100.0, 100.0, 100.0), 1.0, Arc::new(LambertianMaterial {}))),
    ];

    let lights = vec![
        Light::new(Vec::new(5.0, 5.0, 5.0), 500.0),
        Light::new(Vec::new(-5.0, 3.0, 1.0), 400.0),
        Light::new(Vec::new(0.0, 1000.0, 5.0), 1e7),
        Light::new(Vec::new(-0.8, 1.3, 4.1), 2.0),
    ];

    camera.capture(&objects[..], &lights, "out/render.png")
}

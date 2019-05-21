mod color;
mod vector;
mod object;
mod ray;
mod light;
mod film;
mod camera;
mod hittable;
mod material;

use vector::Vec;
use color::Color;
use hittable::*;
use object::sphere::Sphere;
use object::plane::Plane;
use light::Light;
use film::Film;
use camera::Camera;
use material::ReflectiveMaterial;

fn main() {
    let eye = Vec::new(0.0, 0.0, 0.3);
    let film = Film::new(Vec::new(-0.8, 1.2, 1.3), Vec::new(1.2, -0.3, 1.3));
    let camera = Camera { eye, film, img_x: 1600, img_y: 1200, samples: 20 };

    let objects: std::vec::Vec<Box<Hittable>> = vec![
        Box::new(Sphere::new(Vec::new(-1.0, 1.0, 5.0), 0.8, Color::new(255.0, 50.0,  50.0),  0.05, &ReflectiveMaterial {})),
        Box::new(Sphere::new(Vec::new(1.0,  1.0, 5.0), 0.8, Color::new(50.0,  255.0, 100.0), 0.8, &ReflectiveMaterial {})),
        Box::new(Sphere::new(Vec::new(2.5,  1.0, 5.0), 0.8, Color::new(50.0,  100.0, 255.0), 0.0, &ReflectiveMaterial {})),
        Box::new(Sphere::new(Vec::new(-1.0, 2.0, 4.0), 0.2, Color::new(220.0, 220.0, 75.0),  0.7, &ReflectiveMaterial {})),

        Box::new(Plane::new(Vec::new(0.0, -1.0, 0.0), Vec::new(0.0, 1.0, 0.0), Color::new(100.0, 100.0, 100.0), 0.0, &ReflectiveMaterial {})),
    ];

    let lights = vec![
        Light::new(Vec::new(5.0, 5.0, 5.0), 500.0),
        Light::new(Vec::new(-5.0, 3.0, 1.0), 400.0),
        Light::new(Vec::new(0.0, 1000.0, 5.0), 1e7),
        Light::new(Vec::new(-0.8, 1.3, 4.1), 2.0),
    ];

    camera.capture(&objects, &lights, "out/render.png")
}

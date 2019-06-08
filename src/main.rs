mod camera;
mod cli;
mod color;
mod film;
mod hittable;
mod light;
mod material;
mod object;
mod ray;
mod vector;

use camera::Camera;
use color::*;
use film::Film;
use hittable::*;
use light::Light;
use material::*;
use object::plane::Plane;
use object::sphere::Sphere;
use vector::Vec;

fn main() {
    let cli_args = cli::CLI::new();

    let eye = Vec::new(0.0, 0.3, 0.3);
    let film = Film::new(Vec::new(-0.8, 1.0, 1.3), Vec::new(1.2, -0.5, 1.3));
    let camera = Camera {
        eye,
        film,
        img_x: 1600,
        img_y: 1200,
        samples: cli_args.samples(),
    };

    let glass_sphere = Sphere::new(
        Vec::new(-1.0, -0.2, 5.0),
        0.8,
        rgb!(255.0, 255.0, 255.0),
        1.0,
        &DielectricMaterial {
            refractive_index: 1.3,
        },
    );
    let small_glass_sphere = Sphere::new(
        Vec::new(1.2, 0.0, 3.0),
        0.4,
        rgb!(255.0, 255.0, 255.0),
        1.0,
        &DielectricMaterial {
            refractive_index: 1.3,
        },
    );
    let fuzzy_green_sphere = Sphere::new(
        Vec::new(1.0, -0.2, 5.0),
        0.8,
        rgb!(50.0, 255.0, 100.0),
        1.0,
        &FuzzyReflectiveMaterial { fuzz: 0.1 },
    );
    let blue_sphere = Sphere::new(
        Vec::new(2.5, -0.2, 5.0),
        0.8,
        rgb!(50.0, 100.0, 255.0),
        1.0,
        &LambertianMaterial {},
    );
    let yellow_sphere = Sphere::new(
        Vec::new(-1.0, 2.0, 4.0),
        0.2,
        rgb!(220.0, 220.0, 75.0),
        1.0,
        &ReflectiveMaterial {},
    );
    let checkerboard = Plane::new(
        Vec::new(0.0, -1.0, 0.0),
        Vec::new(0.0, 1.0, 0.0),
        rgb!(100.0, 100.0, 100.0),
        1.0,
        &LambertianMaterial {},
    );

    let objects: std::vec::Vec<&(Hittable + Send + Sync)> = vec![
        &glass_sphere,
        &small_glass_sphere,
        &fuzzy_green_sphere,
        &blue_sphere,
        &yellow_sphere,
        &checkerboard,
    ];

    let lights = vec![
        Light::new(Vec::new(5.0, 5.0, 5.0), 500.0),
        Light::new(Vec::new(-5.0, 3.0, 1.0), 400.0),
        Light::new(Vec::new(0.0, 1000.0, 5.0), 1e7),
        Light::new(Vec::new(-0.8, 1.3, 4.1), 2.0),
    ];

    camera.capture(&objects[..], &lights, "out/render.png")
}

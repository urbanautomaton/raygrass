use crate::color::*;
use crate::hittable::Hittable;
use crate::light::Light;
use crate::material::*;
use crate::object::plane::Plane;
use crate::object::sphere::Sphere;
use crate::vector::Vec;

pub struct Scene {
    pub objects: std::vec::Vec<Box<Hittable + Send + Sync>>,
    pub lights: std::vec::Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        let glass_sphere = Sphere::new(
            Vec::new(-1.0, -0.2, 5.0),
            0.8,
            Color::new(255.0, 255.0, 255.0),
            1.0,
            &DielectricMaterial {
                refractive_index: 1.3,
            },
        );
        let small_glass_sphere = Sphere::new(
            Vec::new(1.2, 0.0, 3.0),
            0.4,
            Color::new(255.0, 255.0, 255.0),
            1.0,
            &DielectricMaterial {
                refractive_index: 1.3,
            },
        );
        let fuzzy_green_sphere = Sphere::new(
            Vec::new(1.0, -0.2, 5.0),
            0.8,
            Color::new(50.0, 255.0, 100.0),
            1.0,
            &FuzzyReflectiveMaterial { fuzz: 0.1 },
        );
        let blue_sphere = Sphere::new(
            Vec::new(2.5, -0.2, 5.0),
            0.8,
            Color::new(50.0, 100.0, 255.0),
            1.0,
            &LambertianMaterial {},
        );
        let yellow_sphere = Sphere::new(
            Vec::new(-1.0, 2.0, 4.0),
            0.2,
            Color::new(220.0, 220.0, 75.0),
            1.0,
            &ReflectiveMaterial {},
        );
        let checkerboard = Plane::new(
            Vec::new(0.0, -1.0, 0.0),
            Vec::new(0.0, 1.0, 0.0),
            Color::new(100.0, 100.0, 100.0),
            1.0,
            &LambertianMaterial {},
        );

        let objects: std::vec::Vec<Box<Hittable + Send + Sync>> = vec![
            Box::new(glass_sphere),
            Box::new(small_glass_sphere),
            Box::new(fuzzy_green_sphere),
            Box::new(blue_sphere),
            Box::new(yellow_sphere),
            Box::new(checkerboard),
        ];

        let lights = vec![
            Light::new(Vec::new(5.0, 5.0, 5.0), 500.0),
            Light::new(Vec::new(-5.0, 3.0, 1.0), 400.0),
            Light::new(Vec::new(0.0, 1000.0, 5.0), 1e7),
            Light::new(Vec::new(-0.8, 1.3, 4.1), 2.0),
        ];

        Self { objects, lights }
    }
}

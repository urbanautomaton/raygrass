use rand::prelude::*;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;

use crate::bvh::*;
use crate::color::*;
use crate::hittable::*;
use crate::light::Light;
use crate::material::*;
use crate::object::plane::*;
use crate::object::sphere::*;
use crate::texture::*;
use crate::vector::Vec;

pub struct Scene {
    pub objects: std::vec::Vec<Box<Hittable>>,
    pub lights: std::vec::Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        let glass_sphere = Sphere::new(
            Vec::new(-1.0, 0.8, 5.0),
            0.8,
            ConstantTexture {
                color: Color::new(255.0, 255.0, 255.0),
            },
            1.0,
            &DielectricMaterial {
                refractive_index: 1.3,
            },
        );
        let small_glass_sphere = Sphere::new(
            Vec::new(1.2, 1.5, 3.0),
            0.4,
            ConstantTexture {
                color: Color::new(255.0, 255.0, 255.0),
            },
            1.0,
            &DielectricMaterial {
                refractive_index: 1.3,
            },
        );
        let fuzzy_green_sphere = Sphere::new(
            Vec::new(1.0, 0.8, 5.0),
            0.8,
            ConstantTexture {
                color: Color::new(50.0, 255.0, 100.0),
            },
            1.0,
            &FuzzyReflectiveMaterial { fuzz: 0.1 },
        );
        let blue_sphere = Sphere::new(
            Vec::new(2.5, 0.8, 5.0),
            0.8,
            ConstantTexture {
                color: Color::new(50.0, 100.0, 255.0),
            },
            1.0,
            &LambertianMaterial {},
        );
        let yellow_sphere = Sphere::new(
            Vec::new(1.75, 1.5, 6.2),
            0.5,
            ConstantTexture {
                color: Color::new(220.0, 220.0, 75.0),
            },
            1.0,
            &ReflectiveMaterial {},
        );
        let checkerboard = Plane::new(
            Vec::new(0.0, -0.0, 0.0),
            Vec::new(0.0, 1.0, 0.0),
            ConstantTexture {
                color: Color::new(100.0, 100.0, 100.0),
            },
            1.0,
            &LambertianMaterial {},
        );

        let mut boundeds: std::vec::Vec<Box<dyn BoundedHittable>> = vec![
            Box::new(glass_sphere),
            Box::new(small_glass_sphere),
            Box::new(fuzzy_green_sphere),
            Box::new(blue_sphere),
            Box::new(yellow_sphere),
        ];

        let mut rng = Xoshiro256StarStar::seed_from_u64(0);

        for _ in 1..100 {
            let color_coords: [f64; 3] = rng.gen();
            let material: &Material;

            match rng.gen_range(0u32, 3) {
                0 => material = &LambertianMaterial {},
                1 => material = &FuzzyReflectiveMaterial { fuzz: 0.1 },
                2 => {
                    material = &DielectricMaterial {
                        refractive_index: 1.3,
                    }
                }
                _ => material = &ReflectiveMaterial {},
            }

            boundeds.push(Box::new(Sphere::new(
                Vec::new(rng.gen_range(-5., 5.), 0.1, rng.gen_range(2., 10.)),
                0.1,
                ConstantTexture {
                    color: Color::from(color_coords).scale(255.0),
                },
                1.0,
                material,
            )))
        }

        let objects: std::vec::Vec<Box<Hittable>> =
            vec![Box::new(BVH::new(boundeds)), Box::new(checkerboard)];

        let lights = vec![
            Light::new(Vec::new(5.0, 5.0, 5.0), 500.0),
            Light::new(Vec::new(-5.0, 3.0, 1.0), 400.0),
            Light::new(Vec::new(0.0, 1000.0, 5.0), 1e7),
            Light::new(Vec::new(-0.8, 1.3, 4.1), 2.0),
        ];

        Self { objects, lights }
    }
}

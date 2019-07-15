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
            DielectricMaterial {
                texture: ConstantTexture {
                    color: Color::new(1., 1., 1.),
                },
                refractive_index: 1.3,
            },
        );
        let small_glass_sphere = Sphere::new(
            Vec::new(1.2, 1.5, 3.0),
            0.4,
            DielectricMaterial {
                texture: ConstantTexture {
                    color: Color::new(1., 1., 1.),
                },
                refractive_index: 1.3,
            },
        );
        let fuzzy_green_sphere = Sphere::new(
            Vec::new(1.0, 0.8, 5.0),
            0.8,
            FuzzyReflectiveMaterial {
                texture: ConstantTexture {
                    color: Color::new(0.2, 1., 0.4),
                },
                fuzz: 0.1,
            },
        );
        let blue_sphere = Sphere::new(
            Vec::new(2.5, 0.8, 5.0),
            0.8,
            LambertianMaterial {
                texture: CheckerboardTexture {
                    odd: ConstantTexture {
                        color: Color::new(0.9, 0.9, 0.9),
                    },
                    even: ConstantTexture {
                        color: Color::new(0.2, 0.4, 1.),
                    },
                    pitch: 20.,
                },
            },
        );
        let yellow_sphere = Sphere::new(
            Vec::new(1.75, 1.5, 6.2),
            0.5,
            ReflectiveMaterial {
                texture: ConstantTexture {
                    color: Color::new(0.85, 0.85, 0.3),
                },
            },
        );
        let checkerboard = Plane::new(
            Vec::new(0.0, 0.0, 0.0),
            Vec::new(0.0, 1.0, 0.0),
            LambertianMaterial {
                texture: CheckerboardTexture {
                    odd: ConstantTexture {
                        color: Color::new(0.05, 0.05, 0.2),
                    },
                    even: ConstantTexture {
                        color: Color::new(0.8, 0.8, 0.8),
                    },
                    pitch: 5.,
                },
            },
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
            let texture = ConstantTexture {
                color: Color::from(color_coords),
            };
            let position = Vec::new(rng.gen_range(-5., 5.), 0.1, rng.gen_range(2., 10.));
            let radius = 0.1;

            match rng.gen_range(0u32, 3) {
                0 => boundeds.push(Box::new(Sphere::new(
                    position,
                    radius,
                    LambertianMaterial { texture },
                ))),
                1 => boundeds.push(Box::new(Sphere::new(
                    position,
                    radius,
                    FuzzyReflectiveMaterial { texture, fuzz: 0.1 },
                ))),
                2 => boundeds.push(Box::new(Sphere::new(
                    position,
                    radius,
                    DielectricMaterial {
                        texture,
                        refractive_index: 1.3,
                    },
                ))),
                _ => boundeds.push(Box::new(Sphere::new(
                    position,
                    radius,
                    ReflectiveMaterial { texture },
                ))),
            }
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

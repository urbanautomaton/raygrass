use image::*;
use rand::prelude::*;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;

use crate::bvh::*;
use crate::color::*;
use crate::geometry::*;
use crate::hittable::*;
use crate::material::*;
use crate::object::plane::*;
use crate::object::sphere::*;
use crate::texture::*;

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new(earth: DynamicImage, moon: DynamicImage) -> Self {
        let glass_sphere = Sphere::new(
            Point3::new(-1.0, 0.8, 5.0),
            0.8,
            DielectricMaterial {
                texture: ConstantTexture {
                    color: Color::new(1., 1., 1.),
                },
                refractive_index: 1.3,
            },
        );
        let small_glass_sphere = Sphere::new(
            Point3::new(1.2, 1.5, 3.0),
            0.4,
            DielectricMaterial {
                texture: ConstantTexture {
                    color: Color::new(1., 1., 1.),
                },
                refractive_index: 1.3,
            },
        );
        let fuzzy_green_sphere = Sphere::new(
            Point3::new(1.0, 0.8, 5.0),
            0.8,
            FuzzyReflectiveMaterial {
                texture: ConstantTexture {
                    color: Color::new(0.2, 1., 0.4),
                },
                fuzz: 0.1,
            },
        );
        let blue_sphere = Sphere::new(
            Point3::new(2.5, 0.8, 5.0),
            0.8,
            LambertianMaterial {
                texture: CheckerboardTexture {
                    odd: ConstantTexture {
                        color: Color::new(0.9, 0.9, 0.9),
                    },
                    even: ConstantTexture {
                        color: Color::new(0.2, 0.4, 1.),
                    },
                    width: 0.05,
                },
            },
        );
        let blue_dot = Sphere::new(
            Point3::new(3.5, 1.8, 7.0),
            0.8,
            LambertianMaterial {
                texture: ImageTexture::new(earth),
            },
        );
        let moon = Sphere::new(
            Point3::new(4.5, 2.3, 6.0),
            0.2,
            LambertianMaterial {
                texture: ImageTexture::new(moon),
            },
        );
        let marble_sphere = Sphere::new(
            Point3::new(0.5, 2.5, 6.2),
            0.5,
            LambertianMaterial {
                texture: MarbleTexture::new(10.),
            },
        );
        let yellow_sphere = Sphere::new(
            Point3::new(1.75, 2.5, 6.2),
            0.5,
            ReflectiveMaterial {
                texture: ConstantTexture {
                    color: Color::new(0.85, 0.85, 0.3),
                },
            },
        );
        let checkerboard = Plane::new(
            Point3::new(0.0, 0.0, 0.0),
            Unit3::new(0.0, 0.0, 1.0),
            Unit3::new(1.0, 0.0, 0.0),
            LambertianMaterial {
                texture: CheckerboardTexture {
                    odd: ConstantTexture {
                        color: Color::new(0.05, 0.05, 0.2),
                    },
                    even: ConstantTexture {
                        color: Color::new(0.8, 0.8, 0.8),
                    },
                    width: 1.,
                },
            },
        );

        let mut boundeds: Vec<Box<dyn BoundedHittable>> = vec![
            Box::new(glass_sphere),
            Box::new(small_glass_sphere),
            Box::new(fuzzy_green_sphere),
            Box::new(blue_sphere),
            Box::new(blue_dot),
            Box::new(moon),
            Box::new(marble_sphere),
            Box::new(yellow_sphere),
        ];

        let mut rng = Xoshiro256StarStar::seed_from_u64(0);

        for _ in 1..100 {
            let color_coords: [f64; 3] = rng.gen();
            let texture = ConstantTexture {
                color: Color::from(color_coords),
            };
            let position = Point3::new(rng.gen_range(-5.0..5.), 0.1, rng.gen_range(2.0..10.));
            let radius = 0.1;

            match rng.gen_range(0u32..3) {
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

        let objects: Vec<Box<dyn Hittable>> =
            vec![Box::new(Bvh::new(boundeds)), Box::new(checkerboard)];

        Self { objects }
    }
}

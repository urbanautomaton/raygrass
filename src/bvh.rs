use std::cmp::Ordering;

use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;

use crate::hittable::*;
use crate::ray::*;

pub struct BVH {
    left: Box<BoundedHittable>,
    right: Box<BoundedHittable>,
    bounding_box: BoundingBox,
}

impl BVH {
    fn from_hittables(
        mut hittables: Vec<Box<BoundedHittable>>,
        mut rng: Xoshiro256StarStar,
    ) -> Self {
        let axis = rng.gen_range(0, 3);

        hittables.sort_by(|a, b| {
            a.bounding_box().min[axis]
                .partial_cmp(&b.bounding_box().min[axis])
                .unwrap_or(Ordering::Equal)
        });

        let left: Box<BoundedHittable>;
        let right: Box<BoundedHittable>;

        match hittables.len() {
            1 => panic!("You can't make a BVH of one hittable, buddy."),
            2 => {
                left = hittables.pop().unwrap();
                right = hittables.pop().unwrap();
            }
            3 => {
                left = hittables.pop().unwrap();
                right = Box::new(Self::from_hittables(hittables, rng));
            }
            _ => {
                left = Box::new(Self::new(hittables.split_off(hittables.len() / 2)));
                right = Box::new(Self::from_hittables(hittables, rng));
            }
        }

        Self {
            bounding_box: BoundingBox::combine(&[left.bounding_box(), right.bounding_box()]),
            left: left,
            right: right,
        }
    }

    pub fn new(hittables: Vec<Box<BoundedHittable>>) -> Self {
        let rng = Xoshiro256StarStar::seed_from_u64(0);

        Self::from_hittables(hittables, rng)
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        if self.bounding_box.hit(ray, t_min, t_max) {
            if let Some(l_hit) = self.left.hit(ray, t_min, t_max) {
                if let Some(r_hit) = self.right.hit(ray, t_min, t_max) {
                    if l_hit.t < r_hit.t {
                        Some(l_hit)
                    } else {
                        Some(r_hit)
                    }
                } else {
                    Some(l_hit)
                }
            } else {
                self.right.hit(ray, t_min, t_max)
            }
        } else {
            None
        }
    }
}

impl Bounded for BVH {
    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
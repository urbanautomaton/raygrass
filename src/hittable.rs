use std::mem;

use crate::material::Material;
use crate::ray::Ray;
use crate::geometry::*;

pub struct Hit<'a> {
    pub t: f64,
    pub p: Vec,
    pub u: f64,
    pub v: f64,
    pub normal: Vec,
    pub material: &'a Material,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

pub trait Bounded {
    fn bounding_box(&self) -> BoundingBox;
}

pub trait BoundedHittable: Bounded + Hittable {}
impl<T: Bounded + Hittable> BoundedHittable for T {}

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub min: Vec,
    pub max: Vec,
}

impl BoundingBox {
    pub fn combine(boxes: &[Self]) -> Self {
        boxes[1..].iter().fold(boxes[0], |acc, b| Self {
            min: Vec {
                x: acc.min.x.min(b.min.x),
                y: acc.min.y.min(b.min.y),
                z: acc.min.z.min(b.min.z),
            },
            max: Vec {
                x: acc.max.x.max(b.max.x),
                y: acc.max.y.max(b.max.y),
                z: acc.max.z.max(b.max.z),
            },
        })
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut tmin = t_min;
        let mut tmax = t_max;

        for a in 0..=2 {
            let inv_d = 1. / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
            if t1 < t0 {
                mem::swap(&mut t0, &mut t1);
            }
            tmin = t0.max(tmin);
            tmax = t1.min(tmax);

            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod bounding_box {
        use super::*;
        const SUBJECT: BoundingBox = BoundingBox {
            min: Vec {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            max: Vec {
                x: 1.,
                y: 1.,
                z: 1.,
            },
        };

        fn is_hit(ray: &Ray) -> bool {
            SUBJECT.hit(ray, 0., std::f64::INFINITY)
        }

        #[test]
        fn a_hit_on_x_y_plane() {
            assert!(is_hit(&Ray {
                origin: Vec::new(0.5, 0.5, -1.),
                direction: Vec::new(0., 0., 1.),
            }))
        }

        #[test]
        fn a_hit_for_ray_within_box() {
            assert!(is_hit(&Ray {
                origin: Vec::new(0.5, 0.5, 0.5),
                direction: Vec::new(0., 0., 1.),
            }))
        }

        #[test]
        fn a_ray_pointing_away() {
            assert!(!is_hit(&Ray {
                origin: Vec::new(0.5, 0.5, -1.),
                direction: Vec::new(0., 0., -1.),
            }))
        }

        #[test]
        fn a_glancing_ray() {
            assert!(is_hit(&Ray {
                origin: Vec::new(0., 0., -1.),
                direction: Vec::new(0., 0., 1.),
            }))
        }

        #[test]
        fn a_diagonal_ray() {
            assert!(is_hit(&Ray {
                origin: Vec::new(0., 0., 0.),
                direction: Vec::new(1., 1., 1.),
            }))
        }
    }
}

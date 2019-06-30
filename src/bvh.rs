use crate::hittable::*;
use crate::ray::*;

pub struct BVH {
    left: Box<BoundedHittable>,
    right: Box<BoundedHittable>,
    bounding_box: BoundingBox,
}

impl BVH {
    pub fn new(left: Box<BoundedHittable>, right: Box<BoundedHittable>) -> Self {
        let lbox = left.bounding_box();
        let rbox = right.bounding_box();

        Self {
            left: left,
            right: right,
            bounding_box: BoundingBox::combine(&[lbox, rbox]),
        }
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

impl BoundedHittable for BVH {
    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;

use crate::geometry::*;

pub struct Perlin {
    rands: [Vector3; 256],
    perm_x: [usize; 256],
    perm_y: [usize; 256],
    perm_z: [usize; 256],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = Xoshiro256StarStar::seed_from_u64(0);
        let mut rands: [Vector3; 256] = [Vector3::new(0., 0., 0.); 256];

        for elem in rands.iter_mut() {
            let coords: [f64; 3] = rng.gen();

            *elem = ((Vector3::from(coords) * 2.) - Vector3::new(1., 1., 1.)).normalize();
        }

        let mut xs: Vec<usize> = (0..256).collect();
        let mut ys: Vec<usize> = (0..256).collect();
        let mut zs: Vec<usize> = (0..256).collect();

        xs.shuffle(&mut rng);
        ys.shuffle(&mut rng);
        zs.shuffle(&mut rng);

        let mut perm_x: [usize; 256] = [0; 256];
        let mut perm_y: [usize; 256] = [0; 256];
        let mut perm_z: [usize; 256] = [0; 256];

        perm_x.copy_from_slice(&xs[..256]);
        perm_y.copy_from_slice(&ys[..256]);
        perm_z.copy_from_slice(&zs[..256]);

        Self {
            rands,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    #[allow(clippy::many_single_char_names)]
    pub fn noise(&self, point: &Point3) -> f64 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as usize;
        let j = point.y.floor() as usize;
        let k = point.z.floor() as usize;

        let mut weight_vectors: [[[Vector3; 2]; 2]; 2] = [[[Vector3::new(0., 0., 0.); 2]; 2]; 2];

        for (di, item_i) in weight_vectors.iter_mut().enumerate().take(2) {
            for (dj, item_j) in item_i.iter_mut().enumerate().take(2) {
                for (dk, item_k) in item_j.iter_mut().enumerate().take(2) {
                    let index = self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255];

                    *item_k = self.rands[index];
                }
            }
        }

        Self::perlin_interpolate(&weight_vectors, u, v, w)
    }

    pub fn turbulence(&self, point: &Point3, depth: u32) -> f64 {
        let mut accum = 0.;
        let mut temp_p = *point;
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.;
        }

        accum.abs()
    }

    fn perlin_interpolate(c: &[[[Vector3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);

        let mut accum: f64 = 0.;

        for (i, item_i) in c.iter().enumerate().take(2) {
            for (j, item_j) in item_i.iter().enumerate().take(2) {
                for (k, item) in item_j.iter().enumerate().take(2) {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;

                    let weight_v = Vector3::new(u - i_f, v - j_f, w - k_f);

                    accum += (i_f * uu + (1. - i_f) * (1. - uu))
                        * (j_f * vv + (1. - j_f) * (1. - vv))
                        * (k_f * ww + (1. - k_f) * (1. - ww))
                        * item.dot(weight_v);
                }
            }
        }

        accum
    }
}

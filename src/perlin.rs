use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;

use crate::vector::Vec;

pub struct Perlin {
    rands: [f64; 256],
    perm_x: [u32; 256],
    perm_y: [u32; 256],
    perm_z: [u32; 256],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = Xoshiro256StarStar::seed_from_u64(0);
        let mut rands: [f64; 256] = [0.; 256];

        for elem in rands.iter_mut() {
            let val: f64 = rng.gen();
            *elem = val;
        }

        let mut xs: std::vec::Vec<u32> = (0..256).collect();
        let mut ys: std::vec::Vec<u32> = (0..256).collect();
        let mut zs: std::vec::Vec<u32> = (0..256).collect();

        xs.shuffle(&mut rng);
        ys.shuffle(&mut rng);
        zs.shuffle(&mut rng);

        let mut perm_x: [u32; 256] = [0; 256];
        let mut perm_y: [u32; 256] = [0; 256];
        let mut perm_z: [u32; 256] = [0; 256];

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

    pub fn noise(&self, point: &Vec) -> f64 {
        let mut u = point.x - point.x.floor();
        let mut v = point.y - point.y.floor();
        let mut w = point.z - point.z.floor();

        u = u * u * (3. - 2. * u);
        v = v * v * (3. - 2. * v);
        w = w * w * (3. - 2. * w);

        let i: usize = point.x.floor() as usize;
        let j: usize = point.y.floor() as usize;
        let k: usize = point.z.floor() as usize;

        let mut c: [[[f64; 2]; 2]; 2] = [[[0.; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = (self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]) as usize;

                    c[di][dj][dk] = self.rands[index];
                }
            }
        }

        Self::trilinear_interpolate(&c, u, v, w)
    }

    fn trilinear_interpolate(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum: f64 = 0.;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;

                    accum += (i_f * u + (1. - i_f) * (1. - u))
                        * (j_f * v + (1. - j_f) * (1. - v))
                        * (k_f * w + (1. - k_f) * (1. - w))
                        * c[i][j][k];
                }
            }
        }

        accum
    }
}

use std::ops::{Div, Index, Mul};

use super::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Unit3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Unit3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let length = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();

        Self {
            x: x / length,
            y: y / length,
            z: z / length,
        }
    }

    pub fn dot(&self, other: Unit3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn reflect(&self, other: Unit3) -> Unit3 {
        let dot = self.dot(other);
        let Unit3 {
            x: ox,
            y: oy,
            z: oz,
        } = other;

        Self {
            x: self.x - 2. * dot * ox,
            y: self.y - 2. * dot * oy,
            z: self.z - 2. * dot * oz,
        }
    }

    pub fn reverse(&self) -> Unit3 {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl From<Vector3> for Unit3 {
    fn from(vec: Vector3) -> Self {
        vec.normalize()
    }
}

impl Index<usize> for Unit3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(format!("Index {} out of range for Unit3", index)),
        }
    }
}

impl Mul<f64> for Unit3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::Output::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f64> for Unit3 {
    type Output = Vector3;

    fn div(self, scalar: f64) -> Self::Output {
        Self::Output::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Mul for Unit3 {
    type Output = Vector3;

    fn mul(self, other: Self) -> Self::Output {
        let Unit3 {
            x: ax,
            y: ay,
            z: az,
        } = self;
        let Unit3 {
            x: bx,
            y: by,
            z: bz,
        } = other;

        Self::Output::new(ay * bz - by * az, az * bx - ax * bz, ax * by - ay * bx)
    }
}

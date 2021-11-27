use std::ops::{Add, Index, Mul, Sub};

use super::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Self;

    fn add(self, vector: Vector3) -> Self::Output {
        Self::new(self.x + vector.x, self.y + vector.y, self.z + vector.z)
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Self;

    fn sub(self, vector: Vector3) -> Self::Output {
        Self::new(self.x - vector.x, self.y - vector.y, self.z - vector.z)
    }
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, other: Point3) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Point3 {
    type Output = Vector3;

    fn sub(self, other: Point3) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Point3 {
    type Output = Self;

    fn mul(self, scale: f64) -> Self::Output {
        Self::new(self.x * scale, self.y * scale, self.z * scale)
    }
}

impl Index<usize> for Point3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index {} out of range for Point3", index),
        }
    }
}

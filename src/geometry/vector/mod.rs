use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        *self / self.length()
    }

    pub fn dot(&self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl From<[f64; 3]> for Vector3 {
    fn from(coords: [f64; 3]) -> Self {
        let [x, y, z] = coords;

        Self { x, y, z }
    }
}

impl Index<u64> for Vector3 {
    type Output = f64;

    fn index(&self, index: u64) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(format!("Index {} out of range for Vector3", index)),
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let Vector3 {
            x: ax,
            y: ay,
            z: az,
        } = self;
        let Vector3 {
            x: bx,
            y: by,
            z: bz,
        } = other;

        Self::new(ay * bz - by * az, az * bx - ax * bz, ax * by - ay * bx)
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

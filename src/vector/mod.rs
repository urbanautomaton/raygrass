use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec {
        *self / self.length()
    }

    pub fn dot(&self, other: Vec) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vec {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vec {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul for Vec {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let x = self.y * other.z - other.y * self.z;
        let y = self.x * other.z - other.x * self.z;
        let z = self.x * other.y - other.x * self.y;

        Self::new(x, y, z)
    }
}

impl Div<f64> for Vec {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

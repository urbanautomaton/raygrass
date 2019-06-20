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

impl From<[f64; 3]> for Vec {
    fn from(coords: [f64; 3]) -> Self {
        let [x, y, z] = coords;

        Self { x, y, z }
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
        let Vec {
            x: ax,
            y: ay,
            z: az,
        } = self;
        let Vec {
            x: bx,
            y: by,
            z: bz,
        } = other;

        Self::new(ay * bz - by * az, az * bx - ax * bz, ax * by - ay * bx)
    }
}

impl Div<f64> for Vec {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

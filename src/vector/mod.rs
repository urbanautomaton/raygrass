#[derive(Debug, Copy, Clone)]
pub struct Vec {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
       (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn scale(&self, scalar: f64) -> Vec {
        Vec::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn normalize(&self) -> Vec {
        let len = self.length();

        Vec::new(self.x / len, self.y / len, self.z / len)
    }

    pub fn add(&self, other: Vec) -> Vec {
        Vec::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn subtract(&self, other: Vec) -> Vec {
        Vec::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn dot(&self, other: Vec) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

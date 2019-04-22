#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn scale(&self, scalar: f64) -> Self {
        Self::new(
            (self.r * scalar).min(255.0),
            (self.g * scalar).min(255.0),
            (self.b * scalar).min(255.0),
        )
    }

    pub fn add(&self, other: Self) -> Self {
        Self::new(
            (self.r + other.r).min(255.0),
            (self.g + other.g).min(255.0),
            (self.b + other.b).min(255.0),
        )
    }
}

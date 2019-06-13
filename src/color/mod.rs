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
        Self::new(self.r * scalar, self.g * scalar, self.b * scalar)
    }

    pub fn add(&self, other: Self) -> Self {
        Self::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> Self {
        let clamp = std::u8::MAX as f64;

        [
            color.r.min(clamp) as u8,
            color.g.min(clamp) as u8,
            color.b.min(clamp) as u8,
        ]
    }
}

#[macro_export]
macro_rules! rgb {
    ( $r:expr, $g:expr, $b:expr ) => {{
        Color {
            r: $r,
            g: $g,
            b: $b,
        }
    }};
}

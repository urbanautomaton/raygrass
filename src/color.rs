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

impl From<[f64; 3]> for Color {
    fn from(coords: [f64; 3]) -> Self {
        Color::new(coords[0], coords[1], coords[2])
    }
}

impl From<Color> for [u8; 3] {
    fn from(color: Color) -> Self {
        let max = f64::from(std::u8::MAX);

        [
            (color.r * max).min(max) as u8,
            (color.g * max).min(max) as u8,
            (color.b * max).min(max) as u8,
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

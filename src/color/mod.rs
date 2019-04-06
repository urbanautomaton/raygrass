#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn scale(&self, scalar: f64) -> Self {
        // TODO this is b0rked cos of integer overflow in the cast and presumably in the
        // multiplication too.
        Self::new(
            self.r * scalar as u8,
            self.g * scalar as u8,
            self.b * scalar as u8,
        )
    }
}

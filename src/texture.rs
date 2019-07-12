use crate::color::Color;

pub trait Texture: Send + Sync {
    fn color(&self, u: f64, v: f64) -> Color;
}

pub struct ConstantTexture {
    pub color: Color,
}

impl Texture for ConstantTexture {
    fn color(&self, _u: f64, _v: f64) -> Color {
        self.color
    }
}

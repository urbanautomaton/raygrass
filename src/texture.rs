use crate::color::Color;
use crate::vector::Vec;

pub trait Texture: Send + Sync {
    fn color(&self, u: f64, v: f64, p: &Vec) -> Color;
}

pub struct ConstantTexture {
    pub color: Color,
}

impl Texture for ConstantTexture {
    fn color(&self, _u: f64, _v: f64, _p: &Vec) -> Color {
        self.color
    }
}

pub struct CheckerboardTexture<T1: Texture, T2: Texture> {
    pub odd: T1,
    pub even: T2,
    pub width: f64,
}

impl<T1: Texture, T2: Texture> Texture for CheckerboardTexture<T1, T2> {
    fn color(&self, u: f64, v: f64, p: &Vec) -> Color {
        let pitch = std::f64::consts::PI / self.width;
        let cosines = (pitch * u).cos() * (pitch * v).cos();

        if cosines < 0. {
            self.odd.color(u, v, p)
        } else {
            self.even.color(u, v, p)
        }
    }
}

#[allow(dead_code)]
pub struct UVTexture {}

impl Texture for UVTexture {
    fn color(&self, u: f64, v: f64, _p: &Vec) -> Color {
        Color::new(u, v, 0.5)
    }
}

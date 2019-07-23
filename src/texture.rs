use image::*;

use crate::color::Color;
use crate::perlin::Perlin;
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
        let sines = (pitch * u).sin() * (pitch * v).sin();

        if sines < 0. {
            self.odd.color(u, v, p)
        } else {
            self.even.color(u, v, p)
        }
    }
}

pub struct ImageTexture {
    width: f64,
    height: f64,
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(image: DynamicImage) -> Self {
        Self {
            width: f64::from(image.width()),
            height: f64::from(image.height()),
            image: image.flipv().to_rgb(),
        }
    }
}

impl Texture for ImageTexture {
    fn color(&self, u: f64, v: f64, _p: &Vec) -> Color {
        let x = (self.width * u.fract()) as u32;
        let y = (self.height * v.fract()) as u32;
        let pixel = self.image.get_pixel(x, y);

        Color::new(
            f64::from(pixel[0]) / 255.,
            f64::from(pixel[1]) / 255.,
            f64::from(pixel[2]) / 255.,
        )
    }
}

#[allow(dead_code)]
pub struct UVTexture {}

impl Texture for UVTexture {
    fn color(&self, u: f64, v: f64, _p: &Vec) -> Color {
        Color::new(u, v, 0.5)
    }
}

#[allow(dead_code)]
pub struct NoiseTexture {
    perlin: Perlin,
    scale: f64,
}

impl NoiseTexture {
    #[allow(dead_code)]
    pub fn new(scale: f64) -> Self {
        Self {
            perlin: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn color(&self, _u: f64, _v: f64, p: &Vec) -> Color {
        let scaled_p = *p * self.scale;
        let noise = self.perlin.noise(&scaled_p);

        Color::new(1., 1., 1.).scale(0.5 * (1. + noise))
    }
}

pub struct MarbleTexture {
    perlin: Perlin,
    scale: f64,
}

impl MarbleTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            perlin: Perlin::new(),
            scale,
        }
    }
}

impl Texture for MarbleTexture {
    fn color(&self, _u: f64, _v: f64, p: &Vec) -> Color {
        let scaled_p = *p * self.scale;
        let noise = self.perlin.turbulence(&scaled_p, 7);
        let gray_scale = 0.5 * (1. + (self.scale * p.z.sin() + 10. * noise).sin());

        Color::new(1., 1., 1.).scale(gray_scale)
    }
}

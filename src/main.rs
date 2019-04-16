extern crate image;

mod color;
mod vector;
mod object;
mod ray;
mod light;
mod film;
mod camera;

use vector::Vec;
use color::Color;
use object::sphere::Sphere;
use light::Light;
use film::Film;
use camera::Camera;

fn main() {
    let img_x = 1600;
    let img_y = 1200;

    let eye = Vec::new(0.0, 0.0, 0.3);
    let film = Film::new(Vec::new(-0.8, 1.2, 1.3), Vec::new(1.2, -0.3, 1.3));
    let camera = Camera { eye, film };
    let objects = vec![
        Sphere::new(Vec::new(0.0, 1.0, 5.0), 1.0, Color::new(255.0, 0.0, 150.0)),
        Sphere::new(Vec::new(1.0, 1.0, 5.0), 1.0, Color::new(0.0, 255.0, 0.0)),
        Sphere::new(Vec::new(2.0, 1.0, 5.0), 1.0, Color::new(0.0, 0.0, 255.0)),
        Sphere::new(Vec::new(-1.0, 1.5, 4.0), 0.2, Color::new(255.0, 255.0, 0.0)),
    ];

    let lights = vec![
        Light::new(Vec::new(5.0, 5.0, 5.0), 400.0),
        Light::new(Vec::new(-5.0, 3.0, 1.0), 300.0),
        Light::new(Vec::new(-0.8, 1.3, 4.1), 1.0),
    ];

    let mut buf = image::ImageBuffer::new(img_x, img_y);

    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let ray = camera.trace(x as f64 / img_x as f64, y as f64 / img_y as f64);

        let mut min_t = std::f64::INFINITY;
        let mut min_object: Option<&Sphere> = None;

        for object in &objects {
            if let Some(t) = object.intersect(ray) {
                if t < min_t {
                   min_t = t;
                   min_object = Some(object);
                }
            }
        }

        if let Some(hit) = min_object {
            let intersection = ray.at(min_t);
            let normal = hit.surface_normal(intersection);

            let energy = lights
                .iter()
                .fold(0.0, |acc, light|
                      acc + light.illuminate(intersection, normal, &objects)
                );

            let shade = hit.color.scale(energy);

            *pixel = image::Rgb([shade.r as u8, shade.g as u8, shade.b as u8]);
        } else {
            *pixel = image::Rgb([30, 30, 30]);
        }

    }

    buf.save("out.png").expect("Saving image failed");
}

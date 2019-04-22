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
use ray::Ray;


fn trace(objects: &[Sphere], lights: &[Light], ray: Ray, remaining_calls: u32) -> Option<Color> {
    if remaining_calls <= 0 {
        return None;
    }

    let mut min_t = std::f64::INFINITY;
    let mut min_object: Option<&Sphere> = None;

    for object in objects {
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

        let illuminated_color = hit.color.scale(energy).scale(1.0 - hit.reflectance);

        let dot = ray.direction.dot(normal);
        let reflection_direction = ray.direction.subtract(normal.scale(2.0 * dot));
        let reflection_point = intersection.add(normal.scale(1e-10));
        let reflection_ray = Ray { origin: reflection_point, direction: reflection_direction };

        if let Some(incoming_color) = trace(objects, lights, reflection_ray, remaining_calls - 1) {
            let reflection_color = hit.color.scale(hit.reflectance / 255.0);
            let reflected_color = Color::new(
                incoming_color.r * reflection_color.r,
                incoming_color.g * reflection_color.g,
                incoming_color.b * reflection_color.b,
                );

            Some(illuminated_color.add(reflected_color))
        } else {
            Some(illuminated_color)
        }
    } else {
        None
    }
}

fn main() {
    let img_x = 1600;
    let img_y = 1200;

    let eye = Vec::new(0.0, 0.0, 0.3);
    let film = Film::new(Vec::new(-0.8, 1.2, 1.3), Vec::new(1.2, -0.3, 1.3));
    let camera = Camera { eye, film };

    let objects = vec![
        Sphere::new(Vec::new(-1.0,  1.0, 5.0), 0.8, Color::new(255.0, 50.0,  50.0),  0.2),
        Sphere::new(Vec::new(1.0,   1.0, 5.0), 0.8, Color::new(50.0,  255.0, 100.0), 0.8),
        Sphere::new(Vec::new(2.5, 1.0, 5.0), 0.8, Color::new(50.0,  100.0, 255.0), 0.0),
        Sphere::new(Vec::new(-1.0,  2.0, 4.0), 0.2, Color::new(220.0, 220.0, 75.0),  0.7),
    ];

    let lights = vec![
        Light::new(Vec::new(5.0, 5.0, 5.0), 500.0),
        Light::new(Vec::new(-5.0, 3.0, 1.0), 400.0),
        Light::new(Vec::new(0.0, 1000.0, 5.0), 1e7),
        Light::new(Vec::new(-0.8, 1.3, 4.1), 2.0),
    ];

    let mut buf = image::ImageBuffer::new(img_x, img_y);

    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let ray = camera.trace(x as f64 / img_x as f64, y as f64 / img_y as f64);

        if let Some(color) = trace(&objects, &lights, ray, 50) {
            *pixel = image::Rgb([color.r as u8, color.g as u8, color.b as u8]);
        } else {
            *pixel = image::Rgb([30, 30, 30]);
        }

    }

    buf.save("out/render.png").expect("Saving image failed");
}

mod bvh;
mod camera;
mod cli;
mod color;
mod geometry;
mod hittable;
mod material;
mod object;
mod perlin;
mod ray;
mod scene;
mod texture;

use camera::Camera;
use geometry::*;
use scene::Scene;

#[allow(dead_code)]
fn main() {
    let cli_args = cli::CLI::new();

    let look_from = Vector3::new(0.0, 2.8, 0.3);
    let look_at = Vector3::new(1.0, 0.8, 5.0);

    let (img_x, img_y) = cli_args.resolution();

    let time = cli_args.time();
    let theta = 2. * std::f64::consts::PI * time / 10.;
    let cos = theta.cos();
    let sin = theta.sin();

    let c = look_from - look_at;
    let rotated_look_from =
        Vector3::new(c.x * cos - c.z * sin, c.y - sin, c.x * sin + c.z * cos) + look_at;

    let camera = Camera::new(
        rotated_look_from,
        look_at,
        60.0,
        0.05,
        (look_at - look_from).length(),
        img_x,
        img_y,
    );

    let earth = image::open("resources/earth.png").unwrap();
    let moon = image::open("resources/moon.jpg").unwrap();

    let scene: Scene = Scene::new(earth, moon);

    camera.capture(&scene, cli_args.samples(), cli_args.outfile())
}

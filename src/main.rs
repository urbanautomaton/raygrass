mod camera;
mod cli;
mod color;
mod hittable;
mod light;
mod material;
mod object;
mod ray;
mod scene;
mod vector;

use camera::Camera;
use scene::Scene;
use vector::Vec;

#[allow(dead_code)]
fn main() {
    let cli_args = cli::CLI::new();

    let camera = Camera::new(
        Vec::new(0.0, 3.3, 0.3),
        Vec::new(1.0, 0.8, 5.0),
        60.0,
        1600,
        1200,
        cli_args.samples(),
    );

    let scene = Scene::new();

    camera.capture(&scene, cli_args.outfile())
}

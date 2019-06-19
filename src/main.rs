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

    let look_from = Vec::new(0.0, 3.3, 0.3);
    let look_at = Vec::new(1.0, 0.8, 5.0);

    let camera = Camera::new(
        look_from,
        look_at,
        60.0,
        0.05,
        (look_at - look_from).length() - 0.5,
        1600,
        1200,
        cli_args.samples(),
    );

    let scene = Scene::new();

    camera.capture(&scene, cli_args.outfile())
}

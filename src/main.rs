mod camera;
mod cli;
mod color;
mod film;
mod hittable;
mod light;
mod material;
mod object;
mod ray;
mod scene;
mod vector;

use camera::Camera;
use film::Film;
use scene::Scene;
use vector::Vec;

fn main() {
    let cli_args = cli::CLI::new();

    let eye = Vec::new(0.0, 0.3, 0.3);
    let film = Film::new(Vec::new(-0.8, 1.0, 1.3), Vec::new(1.2, -0.5, 1.3));
    let camera = Camera {
        eye,
        film,
        img_x: 1600,
        img_y: 1200,
        samples: cli_args.samples(),
    };

    let scene = Scene::new();

    camera.capture(&scene, cli_args.outfile())
}

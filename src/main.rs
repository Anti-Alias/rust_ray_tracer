#[macro_use]
extern crate derive_new;
extern crate raster;
mod geom;
mod scene;

use geom::{Vector, Ray, Plane};
use scene::{Scene, Camera};
use raster::{Color, Image};
use std::fs;

fn main() {

    // Creates Camera that will be used in the scene
    let origin = Vector::new(0.0, 0.0, 10.0);
    let dir = Vector::new(0.0, 0.0, -1.0);
    let camera = Camera {
        up: Vector::new(0.0, 1.0, 0.0),
        dist: 1.0,
        eye: Ray { origin, dir },
        frust_width: 10.0,
        frust_height: 10.0
    };

    // Builds scene that will use camera
    let scene = Scene {
        color: Color {r: 100, g: 100, b: 100, a: 255},
        camera,
        objects: Vec::new()
    };

    // Create canvas image
    let mut canvas = Image::blank(512, 512);

    // For a number of frames...
    let frames = 1;
    for frame in 0..frames {

        // Draw image
        scene.render(&mut canvas);

        // Save image
        fs::create_dir_all("images").unwrap();
        let filename: String = format!("images/frame_{}.png", frame);
        raster::save(&canvas, &filename).unwrap();
    }

    println!("{:#?}", camera);
    println!("Done!!!!!");
}
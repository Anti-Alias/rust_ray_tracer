#[macro_use]
extern crate derive_new;

pub mod camera;
pub mod geom;
pub mod scene;

use geom::{Vector, Ray};
use camera::Camera;

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

    println!("{:#?}", camera);
    println!("Done!!!!!");
}
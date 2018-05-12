#[macro_use]
extern crate derive_new;
extern crate raster;
mod geom;
mod scene;

use geom::{Vector, Ray, Plane};
use scene::{Scene, Camera, Sphere, SceneObject, Intersection};
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

    // Creates sphere(s)
    let mut objects: Vec<Box<SceneObject>> = Vec::new();
    objects.push(
        Box::new (
            Sphere {
                pos: Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0
                },
                color: Color::red(),
                radius: 5.0
            }
        )
    );

    let ray = Ray {
        origin: Vector::new(0.0, 0.0, 10.0),
        dir: Vector::new(0.0, 0.0, -1.0)
    };
    let maybe_inter: Option<Intersection> = objects[0].intersect(&ray.to_unit());
        println!("Intersection: {:?}", maybe_inter);

    // Builds scene that will use camera
    let mut scene = Scene {
        color: Color {r: 100, g: 100, b: 100, a: 255},
        camera,
        objects
    };

    // Create canvas image
    let mut canvas = Image::blank(512, 512);

    // For a number of frames...
    let frames = 20;
    for frame in 0..frames {

        println!("Rendering frame {}", frame);

        // Trace scene
        scene.render(&mut canvas);

        // Save image
        fs::create_dir_all("images").unwrap();
        let filename: String = format!("images/frame_{}.png", frame);
        raster::save(&canvas, &filename);

        // Move camera
        scene.camera.eye.origin.z -= 0.5;
    }

    println!("Done!!!!!");
}
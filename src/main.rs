#[macro_use]
extern crate derive_new;
extern crate raster;
mod geom;
mod scene;
mod shape;

use geom::{Vector, Ray, Plane, Intersection};
use shape::{Sphere, Shape};
use scene::{Scene, Camera, Light};
use raster::{Color, Image};
use std::fs;

fn main() {

    // Creates Camera that will be used in the scene
    let origin = Vector::new(0.0, 2.0, 10.0);
    let dir = Vector::new(0.0, 0.0, -1.0);
    let camera = Camera {
        up: Vector::new(0.0, 1.0, 0.0),
        dist: 2.0,
        eye: Ray { origin, dir },
        frust_width: 10.0,
        frust_height: 10.0
    };

    // Creates sphere(s)
    let red = Vector::new(1.0, 0.0, 0.0);
    let blue = Vector::new(0.0, 0.0, 1.0);
    let white = Vector::new(1.0, 1.0, 1.0);
    let mut shapes: Vec<Box<Shape>> = Vec::new();
    shapes.push(
        Box::new (
            Sphere {
                center: Vector {
                    x: 2.0,
                    y: 0.0,
                    z: 0.0
                },
                color: red,
                radius: 4.0
            }
        )
    );

    shapes.push(
        Box::new (
            Sphere {
                center: Vector {
                    x: -2.0,
                    y: 0.0,
                    z: 0.0
                },
                color: blue,
                radius: 5.0
            }
        )
    );

    // Creates lights
    let lights = vec![
        Light {
            position: Vector::new(-20.0, 10.0, 10.0),
            color: white
        }
    ];


    // Builds scene that will use camera
    let mut scene = Scene {
        color_background: Vector::new(0.1, 0.1, 0.1),
        color_ambient: Vector::new(0.1, 0.1, 0.1),
        camera,
        shapes,
        lights
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
#[macro_use]
extern crate derive_new;
extern crate raster;
extern crate rand;
extern crate pad;
extern crate ray_tracer;

pub mod geom;
pub mod scene;
pub mod shape;

use geom::{Vector, Ray};
use shape::{Shape, Sphere, Floor};
use scene::{Scene, Camera, Light};
use raster::{Image};
use std::fs;
use rand::{Rng, SeedableRng, StdRng};
use pad::{PadStr};
use pad::Alignment;

fn rand_sphere<T>(min_pos: Vector, max_pos: Vector, min_radius: f64, max_radius: f64, rng: &mut T) -> Sphere
where T: Rng {

    let center = Vector {
        x: rng.gen_range(min_pos.x, max_pos.y),
        y: rng.gen_range(min_pos.y, max_pos.y),
        z: rng.gen_range(min_pos.z, max_pos.z)
    };
    let radius: f64 = rng.gen_range(min_radius, max_radius);
    let color = Vector {
        x: rng.next_f64(),
        y: rng.next_f64(),
        z: rng.next_f64()
    };
    Sphere {
        center,
        radius,
        color,
        reflectivity: 0.5,
        exponent: 30.0
    }
}

fn main() {

    // Creates Camera that will be used in the scene
    let origin = Vector::new(0.0, 2.0, 30.0);
    let dir = Vector::new(0.0, 0.0, -1.0);
    let camera = Camera {
        up: Vector::new(0.0, 1.0, 0.0),
        near_dist: 10.0,
        far_dist: 1000.0,
        eye: Ray { origin, dir },
        frust_width: 16.0,
        frust_height: 9.0
    };

    // Creates random number generator for creating objects in scene
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng:StdRng = SeedableRng::from_seed(seed);
    let num_shapes = 100;

    // Creates sphere(s) using rng
    let mut shapes: Vec<Box<Shape>> = Vec::new();
    for _ in 0..num_shapes {
        shapes.push(
            Box::new(
                rand_sphere(
                    Vector::new(-5.0, -5.0, -10.0),
                    Vector::new(5.0, 5.0, 10.0),
                    0.3,
                    1.0,
                    &mut rng
                )
            )
        );
    }

    // Adds dome
    shapes.push(
        Box::new(
            Sphere {
                center: Vector::new(0.0, 0.0, 0.0),
                radius: 50.0,
                color: Vector::new(0.0, 1.0, 0.5),
                reflectivity: 0.0,
                exponent: 100.0
            }
        )
    );


    // Adds floor
    shapes.push(
        Box::new (
            Floor {
                position: Vector::new(0.0, -10.0, 0.0),
                color: Vector::new(0.0, 1.0, 0.5),
                reflectivity: 0.0,
                exponent: 15.0
            }
        )
    );


    // Determines number of shapes and their velocities
    let mut velocities = Vec::new();
    for _ in 0..num_shapes {
        let speed = rng.next_f64() * 0.2;
        let rand_vel = Vector::rand(&mut rng) * speed;
        velocities.push(rand_vel);
    }

    // Adds zero valocity for floor
    let zero = Vector::new(0.0, 0.0, 0.0);
    velocities.push(zero);
    velocities.push(zero);

    // Creates lights
    let lights = vec![
        Light {
            position: Vector::new(-20.0, 10.0, 10.0),
            color: Vector::new(1.0, 1.0, 1.0),
            brightness: 400.0
        },
        Light {
            position: Vector::new(20.0, 10.0, 10.0),
            color: Vector::new(1.0, 1.0, 1.0),
            brightness: 400.0
        }
    ];

    // Builds scene that will use camera
    let mut scene = Scene {
        color_background: Vector::new(0.2, 0.2, 0.2),
        color_ambient: Vector::new(0.2, 0.2, 0.2),
        camera,
        shapes,
        lights
    };

    // Create canvas image
    let mut canvas = Image::blank(1920, 1080);

    // For a number of frames...
    let frames = 80;
    for frame in 0..frames {

        println!("Rendering frame {}", frame);

        // Trace scene
        scene.render(&mut canvas);

        // Save image
        fs::create_dir_all("images").unwrap();
        let number_str = format!("{}", frame).pad(5, '0', Alignment::Right, false);
        let filename: String = format!("images/frame_{}.png", number_str);
        raster::save(&canvas, &filename).unwrap();

        for (i, mut shape) in scene.shapes.iter_mut().enumerate() {
            let new_pos = shape.get_position() + velocities[i];
            shape.set_position(&new_pos);
        }
    }

    println!("Done!!!!!");
}
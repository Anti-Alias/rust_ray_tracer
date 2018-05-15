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
use std::f64::consts::{PI};
use rand::{Rng, SeedableRng, StdRng};
use pad::{PadStr};
use pad::Alignment;

fn rand_sphere<T>(min_radius: f64, max_radius: f64, rng: &mut T) -> Sphere
where T: Rng {

    let radius: f64 = rng.gen_range(min_radius, max_radius);
    let color = Vector {
        x: rng.next_f64(),
        y: rng.next_f64(),
        z: rng.next_f64()
    };
    Sphere {
        center: Vector::new(0.0, 0.0, 0.0),
        radius,
        color,
        reflectivity: 0.3,
        exponent: 30.0
    }
}

fn rand_vector<T>(min_pos: Vector, max_pos: Vector, rng: &mut T) -> Vector
where T: Rng {
    Vector {
        x: rng.gen_range(min_pos.x, max_pos.y),
        y: rng.gen_range(min_pos.y, max_pos.y),
        z: rng.gen_range(min_pos.z, max_pos.z)
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
    let num_spheres = 100;

    // Creates sphere(s) using rng
    let mut shapes: Vec<Box<Shape>> = Vec::new();
    for _ in 0..num_spheres {
        shapes.push(
            Box::new(
                rand_sphere(
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
                radius: 100.0,
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
                position: Vector::new(0.0, -5.0, 0.0),
                color: Vector::new(0.0, 1.0, 0.5),
                reflectivity: 0.5,
                exponent: 15.0
            }
        )
    );

    // Determines number of shapes relevant to movement
    let num_moveable = num_spheres;

    // Gets starting positions
    let mut positions = Vec::new();
    for _ in 0..num_moveable {
        let position: Vector = rand_vector(
            Vector::new(-5.0, -5.0, -10.0),
            Vector::new(5.0, 5.0, 10.0),
            &mut rng
        );
        positions.push(position);
    }

    // Determines number of shapes and their velocities
    let mut directions = Vec::new();
    for _ in 0..num_moveable {
        let amount = rng.next_f64() * 30.0;
        let rand_vel = Vector::rand(&mut rng) * amount;
        directions.push(rand_vel);
    }

    // Adds zero valocity for floor
    let zero = Vector::new(0.0, 0.0, 0.0);

    // Creates lights
    let lights = vec![
        Light {
            position: Vector::new(0.0, 22.0, 10.0),
            color: Vector::new(1.0, 1.0, 1.0),
            brightness: 800.0
        }
    ];

    // Builds scene that will use camera
    let mut scene = Scene {
        color_background: Vector::new(0.2, 0.2, 0.2),
        color_ambient: Vector::new(0.2, 0.3, 0.2),
        camera,
        shapes,
        lights,
        bounce_limit: 2
    };

    // Create canvas image
    let mut canvas = Image::blank(1920, 1080);

    // For a number of frames...
    let frames = 160;
    let camera_dist = 35.0;
    for frame in 0..frames {

        println!("Rendering frame {}", frame);

        {
            // Sets camera position
            let t: f64 = (frame as f64) / (frames as f64);
            let theta: f64 = t * PI;
            let sin_theta2 = (theta*2.0).sin();
            let cos_theta2 = (theta*2.0).cos();
            let mut camera = &mut scene.camera;
            camera.eye.origin = Vector {
                x: sin_theta2 * camera_dist,
                y: 10.0 + sin_theta2 * 10.0,
                z: cos_theta2 * camera_dist
            };
            camera.look_at(Vector::new(0.0, 0.0, 0.0));

            // Moves shapes for next frame
            for i in 0..num_moveable {
                let mut shape: &mut Box<Shape> = &mut scene.shapes[i];
                let position: Vector = positions[i];
                let direction: Vector = directions[i];
                let new_pos = position + direction * sin_theta2;
                shape.set_position(&new_pos);
            }
        }

        // Trace scene
        scene.render(&mut canvas);

        // Save image
        fs::create_dir_all("images").unwrap();
        let number_str = format!("{}", frame).pad(5, '0', Alignment::Right, false);
        let filename: String = format!("images/frame_{}.png", number_str);
        raster::save(&canvas, &filename).unwrap();
    }

    println!("Done!!!!!");
}
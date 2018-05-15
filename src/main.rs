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
use std::time::{Duration, Instant};
use std::fs;
use std::f64::consts::{PI};
use rand::{Rng, SeedableRng, StdRng};
use pad::{PadStr};
use pad::Alignment;

fn rand_sphere<T>(min_radius: f64, max_radius: f64, rng: &mut T) -> Sphere
where T: Rng {

    let radius: f64 = rng.gen_range(min_radius, max_radius);
    let color = Vector {
        x: 0.2 + rng.next_f64() * 0.8,
        y: 0.2 + rng.next_f64() * 0.8,
        z: 0.2 + rng.next_f64() * 0.8
    };
    Sphere {
        center: Vector::new(0.0, 0.0, 0.0),
        radius,
        color,
        reflectivity: 0.5,
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
    let num_spheres = 20;

    // Creates sphere(s) using rng
    let mut shapes: Vec<Box<Shape>> = Vec::new();
    for _ in 0..num_spheres {
        shapes.push(
            Box::new(
                rand_sphere(
                    0.3,
                    2.0,
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
                radius: 400.0,
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
                reflectivity: 0.7,
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

    // Adds zero velocity for floor
    let zero = Vector::new(0.0, 0.0, 0.0);

    // Creates lights
    let num_lights = 1;
    let mut lights = Vec::new();
    for _ in 0..num_lights {
        let rand_vec =  Vector::rand(&mut rng) * 4.0;
        lights.push(
            Light {
                position: Vector::new(0.0, 80.0, 0.0) + rand_vec,
                color: Vector::new(1.0, 1.0, 1.0)  / (num_lights as f64),
                brightness: 20000.0
            }
        )
    }

    // Builds scene that will use camera
    let mut scene = Scene {
        color_background: Vector::new(0.2, 0.2, 0.2),
        color_ambient: Vector::new(0.1, 0.1, 0.1),
        camera,
        shapes,
        lights,
        bounce_limit: 2
    };

    // Create canvas image
    let mut canvas = Image::blank(1920, 1080);

    // For a number of frames...
    let frames = 10;
    let camera_dist = 25.0;
    for frame in 0..frames {

        let now = Instant::now();
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

        // Finishes rendering
        println!("Finished frame {} in {} seconds.", frame, now.elapsed().as_secs());

        // Save image
        fs::create_dir_all("images").unwrap();
        let number_str = format!("{}", frame).pad(5, '0', Alignment::Right, false);
        let filename: String = format!("images/frame_{}.png", number_str);
        raster::save(&canvas, &filename).unwrap();
    }

    println!("Done!!!!!");
}
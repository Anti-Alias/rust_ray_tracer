extern crate raster;

use shape::{Shape};
use geom::{Vector, Ray, Plane, Intersection};
use self::raster::{Image, Color};
use std::cmp::Ordering;

pub fn vector_to_color(vector: &Vector) -> Color {
    let r = (vector.x * 255.0) as u8;
    let g = (vector.y * 255.0) as u8;
    let b = (vector.z * 255.0) as u8;
    Color { r, g, b, a: 255 }
}

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub up: Vector,
    pub near_dist: f64,
    pub far_dist: f64,
    pub eye: Ray,
    pub frust_width: f64,
    pub frust_height: f64
}

impl Camera {

    pub fn near_plane(&self) -> Plane {

        // Forces eye to be the the length of 'dist'.
        let eye_to_center: Ray = self.eye.to_len(self.near_dist);

        // Determines 'right' vector
        let right_dir: Vector = eye_to_center
            .dir
            .cross(&self.up)
            .to_len(self.frust_width);

        // Determines the 'up' vector
        let up_dir: Vector = right_dir
            .cross(&eye_to_center.dir)
            .to_len(self.frust_height);

        // Calculates bottom-left corner of near plane
        let center: Vector = eye_to_center.end();
        let bottom_left: Vector = center - (right_dir/2.0) - (up_dir/2.0);

        // Calculates the plane on which to interpolate
        Plane::new(bottom_left, right_dir, up_dir)
    }

    pub fn look_at(&mut self, point: Vector) {
        let origin = self.eye.origin;
        self.eye = Ray::new(origin, point - origin);
    }
}

#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vector,
    pub color: Vector,
    pub brightness: f64
}

#[derive(Debug)]
pub struct Scene {
    pub color_background: Vector,
    pub color_ambient: Vector,
    pub camera: Camera,
    pub shapes: Vec<Box<Shape>>,
    pub lights: Vec<Light>
}

impl Scene {

    /// Writes intersections of a ray with all objects in the scene.
    fn write_intersections(&self, ray: &Ray, intersections: &mut Vec<Intersection>) {

        // For every object in the scene...
        for shape in &self.shapes {

            // Calculates intersection
            let maybe_inter: Option<Intersection> = shape.intersect(ray);

            // If intersection found, add that intersection
            if let Some(inter) = maybe_inter {
                intersections.push(inter);
            }
        }
    }

    /// Returns true if ray intersects with any object in the scene
    fn intersects(&self, ray: &Ray) -> bool {
        for shape in &self.shapes {
            if shape.intersects(ray) {
                return true;
            }
        }
        false
    }

    pub fn render(&self, image: &mut Image) {

        // Gets image size
        let width: i32 = image.width;
        let height: i32 = image.height;

        // Gets camera and eye position
        let camera: &Camera = &self.camera;
        let eye:Vector = camera.eye.origin;

        // Gets plane at which to trace rays
        let plane = self.camera.near_plane();

        // Creates structure that stores intersections.
        let mut intersections = Vec::<Intersection>::new();

        // For all pixels...
        for y in 0..height {
            for x in 0..width {

                // Calculates 'y' value in image.
                // Flips upside-down
                let y2 = height - y - 1;

                // Clears intersections for this run
                intersections.clear();

                // Gets coordinate ratios
                let xr:f64 = (x as f64 + 0.5) / width as f64;
                let yr:f64 = (y as f64 + 0.5) / height as f64;

                // Gets associated point on the plane
                let plane_pos: Vector = plane.interp(xr, yr);

                // Determines direction of eye to plane position.
                let eye_dir: Vector = plane_pos - eye;
                let z_diff: f64 = camera.far_dist / -eye_dir.z;
                let eye_dir = eye_dir * z_diff;

                // Defines the ray to cast through that position
                let ray = Ray { origin: plane_pos, dir: eye_dir };

                // Calculates intersections
                self.write_intersections(&ray, &mut intersections);

                // If intersections were found..
                if intersections.len() > 0 {

                    // Sorts intersections from closest to farthest
                    intersections.sort_unstable_by(|a, b| {
                        if a.t < b.t { Ordering::Less }
                        else if a.t > b.t { Ordering::Greater }
                        else { Ordering::Equal }
                    });

                    // Gets closest one
                    let closest: &Intersection = &intersections[0];

                    // Gets ambient color
                    let ambient_color: Vector = self.color_ambient;

                    // Gets material color
                    let material_color: Vector = closest.color;

                    // Initializes total light color and specular color as zero.
                    let mut total_light_color: Vector = Vector::new(0.0, 0.0, 0.0);
                    let mut total_specular_color: Vector = Vector::new(0.0, 0.0, 0.0);

                    // Sums light color value for all lights
                    let surface_normal_unit: Vector = closest.normal.to_unit();
                    for light in &self.lights {

                        // Skips this light if it is in the shadow.
                        let inter_pos: Vector = closest.position;
                        let light_dir: Vector = light.position - inter_pos;
                        let inter_to_light = Ray {
                            origin: inter_pos,
                            dir: light_dir
                        };
                        if self.intersects(&inter_to_light) { continue; }

                        // Adds light value
                        let light_dir_unit: Vector = light_dir.to_unit();
                        let cos_angle: f64 = surface_normal_unit.dot(&light_dir_unit);
                        let delta_color = (light.color * cos_angle).clamp();
                        let intensity: f64 = 1.0 / (light_dir.len_squared());
                        total_light_color = total_light_color + delta_color*intensity*light.brightness;

                        // Adds specular value
                        let light_dir_unit = -light_dir_unit;
                        let normal = closest.normal.to_unit();
                        let bounce: Vector = light_dir_unit - normal * 2.0 * (normal.dot(&light_dir_unit));
                        let bounce_unit = bounce.to_unit();
                        let eye_dir_unit: Vector = (-eye_dir).to_unit();
                        let cos_angle = (eye_dir_unit.dot(&bounce_unit));
                        let specular: f64 = (cos_angle).powf(closest.exponent);
                        total_specular_color = (total_specular_color + light.color * specular * closest.reflectivity).clamp();
                    }

                    // Calculates final color
                    let final_color: Vector = (material_color * (ambient_color + total_light_color) + total_specular_color).clamp();

                    // Sets current pixel to that color
                    image.set_pixel(x, y2, vector_to_color(&final_color)).unwrap();
                }
                else {
                    image.set_pixel(x, y2, vector_to_color(&self.color_background)).unwrap();
                }
            }
        }
    }
}
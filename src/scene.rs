extern crate raster;

use std::fmt::Debug;
use geom::{Vector, Ray, Plane};
use self::raster::{Image, Color};
use self::raster::error::RasterResult;
use self::raster::editor;
use std::cmp::Ordering;

pub trait SceneObject : Debug {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

#[derive(Debug)]
pub struct Sphere {
    pub pos: Vector,
    pub radius: f64,
    pub color: Color
}

impl Clone for Sphere {
    fn clone(&self) -> Sphere {
        Sphere { pos: self.pos, radius: self.radius, color: self.color.clone() }
    }
}

impl SceneObject for Sphere {

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {

        // Extracts properties
        let r: f64 = self.radius;
        let o: Vector = ray.origin;
        let c: Vector = self.pos;
        let d: Vector = ray.dir;

        // Calculates a, b and c for quadratic formula
        let a: f64 = d.len_squared();
        let b: f64 = 2.0 * (d.x*(o.x - c.x) + d.y*(o.y - c.y) + d.z*(o.z - c.z));
        let c: f64 = c.len_squared() + o.len_squared() - 2.0*(c.dot(&o)) - r * r;

        // Calculates discriminant. Exits early if negative.
        let discriminant: f64 = b*b - 4.0*a*c;
        if discriminant < 0.0 { return None; }

        // Solves for t and returns intersection.
        let t: f64 = -b-discriminant.sqrt() / 2.0*a;
        let intersection = Intersection {
            t,
            meta: IntersectionMeta::Nothing
        };
        Some(intersection)
    }
}

#[derive(Debug)]
pub enum IntersectionMeta {
    Nothing
}

#[derive(Debug)]
pub struct Intersection {
    t: f64,
    meta: IntersectionMeta
}

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub up: Vector,
    pub dist: f64,
    pub eye: Ray,
    pub frust_width: f64,
    pub frust_height: f64
}

impl Camera {

    pub fn near_plane(&self) -> Plane {

        // Forces eye to be the the length of 'dist'.
        let eye_to_center: Ray = self.eye.to_len(self.dist);

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

#[derive(Debug)]
pub struct Scene {
    pub color: Color,
    pub camera: Camera,
    pub objects: Vec<Box<SceneObject>>
}

impl Scene {

    /// Writes intersections of a ray with all objects in the scene.
    fn write_intersections(&self, ray: &Ray, intersections: &mut Vec<Intersection>) {

        // For every object in the scene...
        for obj in &self.objects {

            // Calculates intersection
            let maybe_inter: Option<Intersection> = obj.intersect(ray);

            // If intersection found, add that intersection
            if let Some(inter) = maybe_inter {
                intersections.push(inter);
            }
        }
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
        for y in 0..width {
            for x in 0..height {

                // Clears intersections for this run
                intersections.clear();

                // Gets coordinate ratios
                let xr:f64 = (x as f64 + 0.5) / width as f64;
                let yr:f64 = (y as f64 + 0.5) / height as f64;

                // Gets associated point on the plane
                let plane_pos: Vector = plane.interp(xr, yr);

                // Defines the ray to cast through that position
                let eye_to_surface = Ray { origin: eye, dir: plane_pos - eye };

                // Calculates intersections
                self.write_intersections(&eye_to_surface, &mut intersections);

                // If intersections were found..
                if intersections.len() > 0 {

                    // Sorts list
                    intersections.sort_unstable_by(|a, b| {
                        if a.t < b.t { Ordering::Less }
                        else if a.t > b.t { Ordering::Greater }
                        else { Ordering::Equal }
                    });

                    // Gets closest one
                    let closest: &Intersection = &intersections[0];

                    // Sets current pixel to that color
                    image.set_pixel(x, y, Color::red());
                }
                else {
                    image.set_pixel(x, y, self.color.clone());
                }
            }
        }
    }
}
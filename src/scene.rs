extern crate raster;

use std::fmt::Debug;
use geom::{Vector, Ray, Plane};
use self::raster::{Image, Color};
use self::raster::error::RasterResult;
use self::raster::editor;

pub trait SceneObject : Debug {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

#[derive(Debug)]
pub struct Sphere {
    pos: Vector,
    radius: f64
}

impl SceneObject for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        None
    }
}

pub enum IntersectionMeta {
    Nothing
}

pub struct Intersection {
    t: f64,
    meta: IntersectionMeta
}

#[derive(Debug)]
pub struct Scene {
    pub color: Color,
    pub camera: Camera,
    pub objects: Vec<Box<SceneObject>>
}

impl Scene {
    pub fn render(&self, image: &mut Image) -> RasterResult<()> {

        // Gets image size
        let width: i32 = image.width;
        let height: i32 = image.height;

        // Gets plane at which to trace rays
        let plane = self.camera.near_plane();

        // Fills image with background color
        editor::fill(image, self.color.clone())
    }
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

        // Determines center of near plane.
        let eye_forced: Ray = self.eye.to_len(self.dist);

        // Determines 'right' vector
        let right_dir: Vector = eye_forced
            .dir
            .cross(&self.up)
            .to_len(self.frust_width);

        // Determines the 'up' vector
        let up_dir: Vector = right_dir
            .cross(&eye_forced.dir)
            .to_len(self.frust_height);

        // Calculates bottom-left corner of near plane
        let center: Vector = eye_forced.end();
        let bottom_left: Vector = center - (right_dir/2.0) - (up_dir/2.0);

        // Calculates the plane on which to interpolate
        Plane::new(bottom_left, right_dir, up_dir)
    }

    pub fn look_at(&mut self, point: Vector) {
        let origin = self.eye.origin;
        self.eye = Ray::new(origin, point - origin);
    }
}
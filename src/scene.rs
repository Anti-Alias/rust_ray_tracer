extern crate raster;

use camera::Camera;
use geom::{Vector, Ray, Plane};
use self::raster::{Image, Color};


trait SceneObject {
    fn intersect(ray: &Ray) -> Option<Intersection>;
}

struct Sphere {
    pos: Vector,
    radius: f64
}

struct Intersection {
    pos: Vector,
    color: Color
}

impl SceneObject for Sphere {
    fn intersect(ray: &Ray) -> Option<Intersection> {
        None
    }
}

struct Scene {
    camera: Camera,
    objects: Vec<Box<Sphere>>
}

impl Scene {
    fn render(&self, image: &Image) {

    }
}
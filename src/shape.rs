use std::fmt::Debug;
use geom::{Vector, Ray, Intersection};

const EPSILON: f64 = 0.000001;

pub trait Shape : Debug {
    fn set_position(&mut self, pos: &Vector);
    fn get_position(&self) -> Vector;
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn intersects(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub color: Vector,
    pub reflectivity: f64,
    pub exponent: f64
}

impl Shape for Sphere {

    fn get_position(&self) -> Vector { self.center }

    fn set_position(&mut self, pos: &Vector) {
        self.center = *pos;
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {

        let xd: f64 = ray.dir.x;
        let yd: f64 = ray.dir.y;
        let zd: f64 = ray.dir.z;

        let xc: f64 = self.center.x;
        let yc: f64 = self.center.y;
        let zc: f64 = self.center.z;

        let sr:f64 = self.radius;

        let x0: f64 = ray.origin.x;
        let y0: f64 = ray.origin.y;
        let z0: f64 = ray.origin.z;

        let h = x0 - xc;
        let i = y0 - yc;
        let j = z0 - zc;

        let a: f64 = xd*xd + yd*yd + zd*zd;
        let b: f64 = 2.0 * ( xd*h + yd*i + zd*j );
        let c: f64 = h*h + i*i + j*j - sr*sr;

        let discrim: f64 = b*b - 4.0*a*c;
        if discrim < 0.0 { return None; }

        let two_a = 2.0 * a;
        let sqrt_discrim = discrim.sqrt();
        let mut t: f64 = (-b - sqrt_discrim) / two_a;
        let mut switch = 1.0;
        if t < EPSILON {
            t = (-b + sqrt_discrim) / two_a;
            if t < EPSILON { return None; }
            switch = -1.0;
        }
        if t > 1.0 { return None; }

        let point_on_sphere: Vector = ray.interp(t);
        let normal = (point_on_sphere - self.center) * switch;

        let inter = Intersection {
            t,
            position: point_on_sphere,
            normal,
            reflectivity: self.reflectivity,
            color: self.color,
            exponent: self.exponent
        };
        Some(inter)
    }
}

#[derive(Debug)]
pub struct Floor {
    pub position: Vector,
    pub color: Vector,
    pub reflectivity: f64,
    pub exponent: f64
}

impl Shape for Floor {

    fn get_position(&self) -> Vector { self.position }
    fn set_position(&mut self, pos: &Vector) { self.position = *pos; }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {

        let t: f64 = (self.position.y - ray.origin.y) / ray.dir.y;
        if t < EPSILON { return None }
        else if t > 1.0 { return None};

        let inter = Intersection {
            t,
            position: ray.interp(t),
            normal: Vector::new(0.0, 1.0, 0.0),
            reflectivity: self.reflectivity,
            color: self.color,
            exponent: self.exponent
        };
        Some(inter)
    }
}
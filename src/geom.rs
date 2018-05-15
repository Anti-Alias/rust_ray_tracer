use std::ops::{Add, Sub, Mul, Div, Neg};
use rand::Rng;


pub fn clamp(num: f64) -> f64 {
    if num < 0.0 { 0.0 }
    else if num > 1.0 { 1.0 }
    else { num }
}

#[derive(Copy, Clone, Debug, new)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn len_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn to_len(&self, len: f64) -> Vector {
        let current_len:f64 = self.len();
        let len_ratio:f64 = len / current_len;
        *self * len_ratio
    }

    pub fn to_unit(&self) -> Vector {
        *self * (1.0/self.len())
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y*other.z - self.z*other.y,
            y: -(self.x*other.z - self.z*other.x),
            z: self.x*other.y - self.y*other.x
        }
    }

    pub fn clamp(&self) -> Vector {
        Vector { x: clamp(self.x), y: clamp(self.y), z: clamp(self.z)}
    }

    /// Random unit vector
    pub fn rand<T>(rng: &mut T) -> Vector
    where T: Rng {
        Vector {
            x: rng.gen_range(-1.0, 1.0),
            y: rng.gen_range(-1.0, 1.0),
            z: rng.gen_range(-1.0, 1.0)
        }.to_unit()
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div<Vector> for Vector {
    type Output = Vector;
    fn div(self, other: Vector) -> Vector {
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, s: f64) -> Vector {
        Vector {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, s: f64) -> Vector {
        let inv = 1.0/s;
        self * inv
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        self * -1.0
    }
}

#[derive(Copy, Clone, Debug, new)]
pub struct Ray {
    pub origin: Vector,
    pub dir: Vector
}

impl Ray {

    pub fn interp(&self, t: f64) -> Vector {
        self.origin + self.dir*t
    }

    pub fn end(&self) -> Vector {
        self.origin + self.dir
    }

    pub fn to_unit(&self) -> Ray {
        Ray {
            origin: self.origin,
            dir: self.dir.to_unit()
        }
    }

    pub fn len(&self) -> f64 {
        self.dir.len()
    }

    pub fn len_squared(&self) -> f64 {
        self.dir.len_squared()
    }

    pub fn to_len(&self, len: f64) -> Ray {
        let current_len:f64 = self.len();
        let len_ratio:f64 = len / current_len;
        Ray {
            origin: self.origin,
            dir: self.dir * len_ratio
        }
    }
}

#[derive(Copy, Clone, Debug, new)]
pub struct Plane {
    pub origin: Vector,
    pub u: Vector,
    pub v: Vector
}

impl Plane {
    pub fn interp(&self, us: f64, vs: f64) -> Vector {
        self.origin + self.u*us + self.v*vs
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub t: f64,
    pub position: Vector,
    pub normal: Vector,
    pub color: Vector,
    pub reflectivity: f64,
    pub exponent: f64
}
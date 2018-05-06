use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Copy, Clone, Debug, new)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn len_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn to_len(&self, len: f64) -> Vec3 {
        let current_len:f64 = self.len();
        let len_ratio:f64 = len / current_len;
        *self * len_ratio
    }

    pub fn to_unit(&self) -> Vec3 {
        *self * (1.0/self.len())
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y*other.z - self.z*other.y,
            y: -(self.x*other.z - self.z*other.x),
            z: self.x*other.y - self.y*other.x
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3{
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f64) -> Vec3 {
        Vec3{
            x: self.x * s,
            y: self.y * s,
            z: self.z * s
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, s: f64) -> Vec3 {
        let inv = 1.0/s;
        self * inv
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        self * -1.0
    }
}

#[derive(Copy, Clone, Debug, new)]
pub struct Ray3 {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray3 {

    pub fn interp(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }

    pub fn end(&self) -> Vec3 {
        self.origin + self.dir
    }

    pub fn to_unit(&self) -> Ray3 {
        Ray3 {
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

    pub fn to_len(&self, len: f64) -> Ray3 {
        let current_len:f64 = self.len();
        let len_ratio:f64 = len / current_len;
        Ray3 {
            origin: self.origin,
            dir: self.dir * len_ratio
        }
    }
}

#[derive(Copy, Clone, Debug, new)]
pub struct Plane {
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3
}

impl Plane {
    pub fn interp(&self, us: f64, vs: f64) -> Vec3 {
        self.origin + self.u*us + self.v*vs
    }
}
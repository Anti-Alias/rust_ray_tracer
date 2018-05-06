use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Copy, Clone, Default)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    fn len_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    fn dot(&self, other: &Vec3) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    fn cross(&self, other: &Vec3) -> Vec3 {
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
use geom::{Vector, Ray, Plane};

#[derive(Debug)]
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
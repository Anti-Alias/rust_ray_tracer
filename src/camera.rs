extern crate raster;

use geom::{Vec3, Ray3, Plane};
use self::raster::{Image};

#[derive(Debug)]
struct Camera {
    pub up: Vec3,
    pub eye: Ray3,
    pub dist: f64,
    pub frust_width: f64,
    pub frust_height: f64
}

impl Camera {
    fn near_plane(&self, image: &mut Image) -> Plane {

        // Determines center of near plane.
        let eye_forced: Ray3 = self.eye.to_len(self.dist);

        // Determines 'right' vector
        let right_dir: Vec3 = eye_forced
            .dir
            .cross(&self.up)
            .to_len(self.frust_width);

        // Determines the 'up' vector
        let up_dir: Vec3 = right_dir
            .cross(&eye_forced.dir)
            .to_len(self.frust_height);

        // Calculates bottom-left corner of near plane
        let center: Vec3 = eye_forced.end();
        let bottom_left: Vec3 = center - (right_dir/2.0) - (up_dir/2.0);

        // Calculates the plane on which to interpolate
        Plane::new(bottom_left, right_dir, up_dir)
    }
}
use crate::{Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    llc: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        const RATIO: f64 = 16.0 / 9.0;
        const VP_HEIGHT: f64 = 2.0;
        const VP_WIDTH: f64 = RATIO * VP_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        let origin = Vec3::zero();
        Camera {
            origin,
            llc: origin + Vec3::new(-VP_WIDTH / 2.0, -VP_HEIGHT / 2.0, -FOCAL_LENGTH),
            horizontal: Vec3::new(VP_WIDTH, 0, 0),
            vertical: Vec3::new(0, VP_HEIGHT, 0),
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.llc + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

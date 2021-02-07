use crate::{Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    llc: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

#[derive(Default)]
pub struct CameraBuilder {
    origin: Option<Vec3>,
    target: Option<Vec3>,
    vup: Option<Vec3>,
    v_fov: Option<f64>,
    aspect_ratio: Option<f64>,
}

impl CameraBuilder {
    pub fn origin(&mut self, value: Vec3) -> &mut Self {
        self.origin = Some(value);
        self
    }
    pub fn target(&mut self, value: Vec3) -> &mut Self {
        self.target = Some(value);
        self
    }
    pub fn vup(&mut self, value: Vec3) -> &mut Self {
        self.vup = Some(value);
        self
    }
    pub fn v_fov(&mut self, value: f64) -> &mut Self {
        self.v_fov = Some(value);
        self
    }
    pub fn aspect_ratio(&mut self, value: f64) -> &mut Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn build(&self) -> Result<Camera, String> {
        let origin = self.origin.ok_or(String::from("`origin` must be initialized"))?;
        let target = self.target.ok_or(String::from("`target` must be initialized"))?;
        let vup = self.vup.ok_or(String::from("`vup` must be initialized"))?;
        let v_fov = self.v_fov.ok_or(String::from("`v_fov` must be initialized"))?;
        let aspect_ratio = self.aspect_ratio.ok_or(String::from("`aspect_ratio` must be initialized"))?;
        Ok(Camera::new(origin, target, vup, v_fov, aspect_ratio))
    }
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }

    /// `v_fov` is in radians.
    fn new(origin: Vec3, target: Vec3, vup: Vec3, v_fov: f64, aspect_ratio: f64) -> Camera {
        let height = 2.0 * (v_fov / 2.0).tan();
        let width = aspect_ratio * height;

        let w = (origin - target).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let horizontal = u * width;
        let vertical = v * height;
        Camera {
            origin,
            horizontal,
            vertical,
            llc: origin - horizontal / 2 - vertical / 2 - w,
        }
    }

    pub fn ray(&self, h: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.llc + self.horizontal * h + self.vertical * v - self.origin,
        )
    }
}

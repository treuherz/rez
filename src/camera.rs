use rand::{distributions::Uniform, Rng};

use crate::{Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    llc: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    offset_distr: Uniform<f64>,
    u: Vec3,
    v: Vec3,
}

#[derive(Default)]
pub struct CameraBuilder {
    origin: Option<Vec3>,
    target: Option<Vec3>,
    vup: Option<Vec3>,
    v_fov: Option<f64>,
    aspect_ratio: Option<f64>,
    aperture: Option<f64>,
    focus_dist: Option<f64>,
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
    pub fn aperture(&mut self, value: f64) -> &mut Self {
        self.aperture = Some(value);
        self
    }
    pub fn focus_dist(&mut self, value: f64) -> &mut Self {
        self.focus_dist = Some(value);
        self
    }

    pub fn build(&self) -> Result<Camera, String> {
        let origin = self
            .origin
            .ok_or_else(|| String::from("`origin` must be initialized"))?;
        let target = self
            .target
            .ok_or_else(|| String::from("`target` must be initialized"))?;
        let vup = self
            .vup
            .ok_or_else(|| String::from("`vup` must be initialized"))?;
        let v_fov = self
            .v_fov
            .ok_or_else(|| String::from("`v_fov` must be initialized"))?;
        let aspect_ratio = self
            .aspect_ratio
            .ok_or_else(|| String::from("`aspect_ratio` must be initialized"))?;
        let aperture = self
            .aperture
            .ok_or_else(|| String::from("`aperture` must be initialized"))?;
        let focus_dist = self
            .focus_dist
            .ok_or_else(|| String::from("`focus_dist` must be initialized"))?;

        let height = 2.0 * (v_fov / 2.0).tan();
        let width = aspect_ratio * height;

        let w = (origin - target).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let horizontal = u * width * focus_dist;
        let vertical = v * height * focus_dist;
        let llc = origin - horizontal / 2 - vertical / 2 - w * focus_dist;

        let lens_radius = aperture / 2.0;
        let offset_distr = Uniform::new_inclusive(-lens_radius, lens_radius);

        Ok(Camera {
            origin,
            horizontal,
            vertical,
            llc,
            offset_distr,
            u,
            v,
        })
    }
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }

    pub fn ray(&self, h: f64, v: f64) -> Ray {
        let u_offset = rand::thread_rng().sample(self.offset_distr);
        let v_offset = rand::thread_rng().sample(self.offset_distr);
        let offset = self.u * u_offset + self.v * v_offset;

        let origin = self.origin + offset;
        Ray::new(
            origin,
            self.llc + self.horizontal * h + self.vertical * v - origin,
        )
    }
}

use crate::{Ray, Collision, Colour, Vec3};

pub trait Material {
    fn scatter(&self, ray: Ray, collision: &Collision) -> Option<(Colour, Ray)>;
}

impl <'a, M> Material for &'a M where M: Material {
    fn scatter(&self, ray: Ray, collision: &Collision) -> Option<(Colour, Ray)> {
        (*self).scatter(ray, collision)
    }
}

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, col: &Collision) -> Option<(Colour, Ray)> {
        let dir = loop {
            let dir = col.normal + rand::random::<Vec3>().unit();
            if !dir.small() {
                break dir;
            }
        };
        let scattered = Ray::new(col.point, dir);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}


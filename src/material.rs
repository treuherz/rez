use crate::{Collision, Colour, Ray, Vec3};

pub trait Material {
    fn scatter(&self, ray: Ray, collision: &Collision) -> Option<(Colour, Ray)>;
}

impl<'a, M> Material for &'a M
where
    M: Material,
{
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
            let dir = rand::random::<Vec3>().unit().ensure_in_hemisphere(col.normal);
            if !dir.small() {
                break dir;
            }
        };
        let scattered = Ray::new(col.point, dir);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Colour,
}

impl Metal {
    pub fn new(albedo: Colour) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, col: &Collision) -> Option<(Colour, Ray)> {
        let reflected = ray.dir.unit().reflect(col.normal);
        let scattered = Ray::new(col.point, reflected);
        let attenuation = self.albedo;
        if reflected.dot(col.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

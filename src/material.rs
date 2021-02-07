use std::ops::Neg;

use rand::random;

use crate::{Collision, Colour, Ray, Vec3};

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - normal * incident.dot(normal) * 2.0
}

fn refract(incident: Vec3, normal: Vec3, eta_ratio: f64) -> Vec3 {
    let cos_theta = incident.unit().neg().dot(normal);
    let transmitted_perp = (incident.unit() + normal * cos_theta) * eta_ratio;
    let transmitted_para = normal * (1.0 - transmitted_perp.squared()).sqrt().neg();
    transmitted_perp + transmitted_para
}

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
            let dir = random::<Vec3>().unit().ensure_in_hemisphere(col.normal);
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, col: &Collision) -> Option<(Colour, Ray)> {
        let reflected = reflect(ray.dir, col.normal);
        let dir = reflected + random::<Vec3>().unit() * self.fuzz;
        let scattered = Ray::new(col.point, dir);
        let attenuation = self.albedo;
        if reflected.dot(col.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    eta: f64,
}

impl Dielectric {
    pub fn new(eta: f64) -> Self {
        Dielectric { eta }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, col: &Collision) -> Option<(Colour, Ray)> {
        // Are we outside the material?
        let eta_ratio = if col.front {
            self.eta.recip()
        } else {
            self.eta
        };

        let cos_theta = ray.dir.unit().neg().dot(col.normal);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let internal_reflection = eta_ratio * sin_theta > 1.0;
        let other_reflection = reflectance(cos_theta, eta_ratio) > random::<f64>();

        let direction = if internal_reflection || other_reflection {
            reflect(ray.dir, col.normal)
        } else {
            refract(ray.dir, col.normal, eta_ratio)
        };

        let attenuation = Colour::new(1.0, 1.0, 1.0);
        let scattered = Ray::new(col.point, direction);
        Some((attenuation, scattered))
    }
}

fn reflectance(cosine: f64, eta: f64) -> f64 {
    let r0 = ((1.0 - eta) / (1.0 + eta)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

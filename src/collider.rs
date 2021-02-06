use crate::{Colour, Material, Ray, Vec3};
use std::fmt::Debug;

pub struct Collision<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front: bool,
    pub material: &'a dyn Material,
}

impl Collision<'_> {
    fn from_ray(ray: Ray, t: f64, outward_normal: Vec3, material: &dyn Material) -> Collision {
        let front = ray.dir.dot(outward_normal) < 0.0;
        let normal = if front {
            outward_normal
        } else {
            -outward_normal
        };
        Collision {
            point: ray.at(t),
            normal,
            t,
            front,
            material,
        }
    }

    pub fn scatter(&self, ray: Ray) -> Option<(Colour, Ray)> {
        self.material.scatter(ray, &self)
    }
}

pub trait Collider {
    fn collide(&self, ray: Ray, t_range: (f64, f64)) -> Option<Collision>;
}

#[derive(Clone, Debug)]
pub struct Sphere<'a, M: 'a + Material> {
    pub centre: Vec3,
    pub radius: f64,
    pub material: &'a M,
}

impl<'a, M: 'a + Material> Sphere<'a, M> {
    pub fn new(centre: Vec3, radius: f64, material: &'a M) -> Self {
        Sphere {
            centre,
            radius,
            material,
        }
    }
}

impl<'a, M: 'a + Material> Collider for Sphere<'a, M> {
    fn collide(&self, ray: Ray, t_range: (f64, f64)) -> Option<Collision> {
        let a = ray.dir.squared();
        let h = (ray.orig - self.centre).dot(ray.dir); // h = b/2
        let c = (ray.orig - self.centre).squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let (t1, t2) = (
            (-h - discriminant.sqrt()) / a,
            (-h + discriminant.sqrt()) / a,
        );
        let t = if t_range.0 <= t1 && t1 <= t_range.1 {
            t1
        } else if t_range.0 <= t2 && t2 <= t_range.1 {
            t2
        } else {
            return None;
        };

        Some(Collision::from_ray(
            ray,
            t,
            (ray.at(t) - self.centre) / self.radius,
            self.material,
        ))
    }
}

impl<'a> Collider for &Vec<Box<dyn Collider + 'a>> {
    fn collide(&self, ray: Ray, t_range: (f64, f64)) -> Option<Collision> {
        self.iter()
            .filter_map(|e| e.collide(ray, t_range))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}

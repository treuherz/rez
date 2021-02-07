use std::{fmt::Debug, marker::PhantomData};

use crate::{Colour, Material, Ray, Vec3};

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
pub struct Sphere<M, RM>
where
    M: Material,
    RM: AsRef<M>,
{
    pub centre: Vec3,
    pub radius: f64,
    pub material: RM,
    phantom: PhantomData<M>,
}

impl<M: Material, RM: AsRef<M>> Sphere<M, RM> {
    pub fn new(centre: Vec3, radius: f64, material: RM) -> Self {
        Sphere {
            centre,
            radius,
            material,
            phantom: PhantomData,
        }
    }
}

impl<M: Material, RM: AsRef<M>> Collider for Sphere<M, RM> {
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
            self.material.as_ref(),
        ))
    }
}

impl Collider for &Vec<Box<dyn Collider + Send + Sync>> {
    fn collide(&self, ray: Ray, t_range: (f64, f64)) -> Option<Collision> {
        self.iter()
            .filter_map(|e| e.collide(ray, t_range))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}

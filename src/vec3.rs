use std::{
    f64::consts::PI,
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use lazy_static::lazy_static;
use rand::{
    distributions::{Distribution, Standard, Uniform},
    Rng,
};
use rand_distr::StandardNormal;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn length(&self) -> f64 {
        self.squared().sqrt()
    }

    pub fn squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn small(&self) -> bool {
        self.length() < 1e-16
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn ensure_in_hemisphere(self, axis: Vec3) -> Vec3 {
        if self.dot(axis) > 0.0 {
            self
        } else {
            -self
        }
    }

    pub fn random_unit<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
        Vec3::new(
            rng.sample::<f64, _>(StandardNormal),
            rng.sample::<f64, _>(StandardNormal),
            rng.sample::<f64, _>(StandardNormal),
        )
        .unit()
    }

    pub fn random_in_unit_sphere<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
        lazy_static! {
            static ref DIST: Uniform<f64> = Uniform::new(-1.0, 1.0);
        }
        loop {
            let v = Vec3::new(
                rng.sample::<f64, _>(*DIST),
                rng.sample::<f64, _>(*DIST),
                rng.sample::<f64, _>(*DIST),
            );
            if v.squared() <= 1.0 {
                break v;
            }
        }
    }

    pub fn random_in_unit_disk<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
        lazy_static! {
            static ref DIST: Uniform<f64> = Uniform::new(0.0, 2.0 * PI);
        }
        let theta = rng.sample::<f64, _>(*DIST);
        let r = rng.gen::<f64>();
        Vec3::new(r * theta.cos(), r * theta.sin(), 0.0)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<N> Mul<N> for Vec3
where
    N: Into<f64>,
{
    type Output = Vec3;

    fn mul(self, rhs: N) -> Self::Output {
        let rhs = rhs.into();
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<N> MulAssign<N> for Vec3
where
    N: Into<f64>,
{
    fn mul_assign(&mut self, rhs: N) {
        *self = *self * rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self * -1
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<N> Div<N> for Vec3
where
    N: Into<f64>,
{
    type Output = Vec3;

    fn div(self, rhs: N) -> Self::Output {
        let rhs = rhs.into();
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<N> DivAssign<N> for Vec3
where
    N: Into<f64>,
{
    fn div_assign(&mut self, rhs: N) {
        *self = *self / rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3::random_in_unit_sphere(rng)
    }
}

use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, UniformFloat, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};
use rand_distr::StandardNormal;
use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Vec3 {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

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
        Vec3::new(
            rng.sample::<f64, _>(StandardNormal),
            rng.sample::<f64, _>(StandardNormal),
            rng.sample::<f64, _>(StandardNormal),
        )
        .unit()
            * rng.gen::<f64>().cbrt()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UniformVec3 {
    x: UniformFloat<f64>,
    y: UniformFloat<f64>,
    z: UniformFloat<f64>,
}

impl UniformSampler for UniformVec3 {
    type X = Vec3;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformVec3 {
            x: UniformFloat::new(low.borrow().x, high.borrow().x),
            y: UniformFloat::new(low.borrow().y, high.borrow().y),
            z: UniformFloat::new(low.borrow().z, high.borrow().z),
        }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformVec3 {
            x: UniformFloat::new_inclusive(low.borrow().x, high.borrow().x),
            y: UniformFloat::new_inclusive(low.borrow().y, high.borrow().y),
            z: UniformFloat::new_inclusive(low.borrow().z, high.borrow().z),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Vec3 {
            x: self.x.sample(rng),
            y: self.y.sample(rng),
            z: self.z.sample(rng),
        }
    }
}

impl SampleUniform for Vec3 {
    type Sampler = UniformVec3;
}

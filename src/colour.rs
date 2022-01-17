use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

use rand::distributions::{Distribution, Standard};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b }
    }

    pub fn to_24bit_rgb(&self) -> (u8, u8, u8) {
        let f = |v: f64| (255.0 * v.clamp(0.0, 1.0)).round() as u8;
        (f(self.r), f(self.g), f(self.b))
    }

    pub const ZERO: Colour = Colour {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    pub const BLACK: Colour = Colour::ZERO;

    pub const WHITE: Colour = Colour {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub fn scale(&self, rhs: Colour) -> Colour {
        Colour {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Self) -> Self::Output {
        Colour {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<N> Mul<N> for Colour
where
    N: Into<f64>,
{
    type Output = Colour;

    fn mul(self, rhs: N) -> Self::Output {
        let rhs = rhs.into();
        Colour {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl<N> MulAssign<N> for Colour
where
    N: Into<f64>,
{
    fn mul_assign(&mut self, rhs: N) {
        *self = *self * rhs;
    }
}

impl<N> Div<N> for Colour
where
    N: Into<f64>,
{
    type Output = Colour;

    fn div(self, rhs: N) -> Self::Output {
        let rhs = rhs.into();
        Colour {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl<N> DivAssign<N> for Colour
where
    N: Into<f64>,
{
    fn div_assign(&mut self, rhs: N) {
        *self = *self / rhs;
    }
}

impl Distribution<Colour> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Colour {
        Colour::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
    }
}

pub struct Blend(pub Colour, pub Colour);

impl Blend {
    pub fn at(&self, t: f64) -> Colour {
        self.0 * (1.0 - t) + self.1 * t
    }
}

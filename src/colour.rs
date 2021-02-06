use std::{
    fmt,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Colour {
    r: f64,
    g: f64,
    b: f64,
    samples: u32,
}

impl Colour {
    pub fn new<R: Into<f64>, B: Into<f64>, G: Into<f64>>(r: R, g: B, b: G) -> Colour {
        Colour {
            r: r.into(),
            g: g.into(),
            b: b.into(),
            samples: 1,
        }
    }

    pub fn zero() -> Colour {
        Colour {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            samples: 0,
        }
    }

    pub fn scale(&self, rhs: Colour) -> Colour {
        Colour {
            r: self.r * rhs.r,
            g: self.g * rhs.r,
            b: self.b * rhs.b,
            samples: self.samples,
        }
    }

    pub fn reset_samples(mut self) -> Colour {
        self.samples = 1;
        self
    }
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Self) -> Self::Output {
        Colour {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            samples: self.samples + rhs.samples,
        }
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sum for Colour {
    fn sum<I: Iterator<Item = Colour>>(iter: I) -> Self {
        iter.fold(Colour::zero(), Add::add)
    }
}

impl Sub for Colour {
    type Output = Colour;

    fn sub(self, rhs: Self) -> Self::Output {
        Colour {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            samples: self.samples - rhs.samples,
        }
    }
}

impl SubAssign for Colour {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
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
            samples: self.samples,
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
            samples: self.samples,
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

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const GAMMA: f64 = 2.0;

        let pixel_val = |v: f64| {
            let scaled = (v / self.samples as f64).powf(GAMMA.recip());
            (255.0 * scaled.clamp(0.0, 1.0)).round() as u8
        };

        let (r, g, b) = (pixel_val(self.r), pixel_val(self.g), pixel_val(self.b));

        write!(f, "{} {} {}", r, g, b)
    }
}

pub struct Blend(pub Colour, pub Colour);

impl Blend {
    pub fn at(&self, t: f64) -> Colour {
        (self.0 * (1.0 - t) + self.1 * t).reset_samples()
    }
}

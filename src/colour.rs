use std::{
    fmt,
    ops::{
        Add,
        AddAssign,
        Div,
        DivAssign,
        Mul,
        MulAssign,
        Sub,
        SubAssign,
    },
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new<R: Into<f64>, B: Into<f64>, G: Into<f64>>(r: R, g: B, b: G) -> Colour {
        Colour { r: r.into(), g: g.into(), b: b.into() }
    }
}

impl Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Self) -> Self::Output {
        Colour { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Colour {
    type Output = Colour;

    fn sub(self, rhs: Self) -> Self::Output {
        Colour { r: self.r - rhs.r, g: self.g - rhs.g, b: self.b - rhs.b }
    }
}

impl SubAssign for Colour {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<N> Mul<N> for Colour where N: Into<f64> {
    type Output = Colour;

    fn mul(self, rhs: N) -> Self::Output {
        let rhs = rhs.into();
        Colour { r: self.r * rhs, g: self.g * rhs, b: self.b * rhs }
    }
}

impl<N> MulAssign<N> for Colour where N: Into<f64> {
    fn mul_assign(&mut self, rhs: N) {
        *self = *self * rhs;
    }
}

impl<N> Div<N> for Colour where N: Into<f64> {
    type Output = Colour;

    fn div(self, rhs: N) -> Self::Output {
        let rhs = rhs.into();
        Colour { r: self.r / rhs, g: self.g / rhs, b: self.b / rhs }
    }
}

impl<N> DivAssign<N> for Colour where N: Into<f64> {
    fn div_assign(&mut self, rhs: N) {
        *self = *self / rhs;
    }
}


impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = (255.0 * self.r.clamp(0.0, 1.0)).round() as u8;
        let g = (255.0 * self.g.clamp(0.0, 1.0)).round() as u8;
        let b = (255.0 * self.b.clamp(0.0, 1.0)).round() as u8;
        write!(f, "{} {} {}", r, g, b)
    }
}

pub struct Blend(pub Colour, pub Colour);

impl Blend {
    pub fn at(&self, t: f64) -> Colour {
        self.0 * (1.0 - t) + self.1 * t
    }
}

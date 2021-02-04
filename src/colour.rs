use std::ops::{Add, Mul, MulAssign, Div, DivAssign, AddAssign, Neg, Sub, SubAssign, Range, RangeInclusive};
use num::Num;
use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {

    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour { r, g, b }
    }

    pub fn from_normalised(r: f64, g: f64, b: f64) -> Colour {
        const NORM: RangeInclusive<f64> = (0f64..=1f64);
        if !NORM.contains(&r) || !NORM.contains(&g) || !NORM.contains(&b) {
            panic!("colour components must be in [0, 1]")
        }
        Colour {
            r: (255.0 * r).round() as u8,
            g: (255.0 * g).round() as u8,
            b: (255.0 * b).round() as u8,
        }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
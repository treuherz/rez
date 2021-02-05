use std::{
    fmt::{
        self,
    },
    ops::RangeInclusive,
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Colour {
    r: f64,
    g: f64,
    b: f64,
}

impl Colour {
    pub fn new<R: Into<f64>, B: Into<f64>, G: Into<f64>>(r: R, g: B, b: G) -> Colour {
        let (r, g, b) = (r.into(), g.into(), b.into());
        const NORM: RangeInclusive<f64> = 0f64..=1f64;
        if !NORM.contains(&r) || !NORM.contains(&g) || !NORM.contains(&b) {
            panic!("colour components must be in [0, 1]")
        }
        Colour { r, g, b }
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = (255.0 * self.r).round() as u8;
        let g = (255.0 * self.g).round() as u8;
        let b = (255.0 * self.b).round() as u8;
        write!(f, "{} {} {}", r, g, b)
    }
}

pub struct Blend(pub Colour, pub Colour);

impl Blend {
    pub fn at(&self, t: f64) -> Colour {
        Colour {
            r: (1.0 - t) * self.0.r + t * self.1.r,
            b: (1.0 - t) * self.0.b + t * self.1.b,
            g: (1.0 - t) * self.0.g + t * self.1.g,
        }
    }
}

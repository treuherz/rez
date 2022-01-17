use crate::Colour;
use std::io;

pub fn encode_ppm(
    data: &[Colour],
    width: u32,
    height: u32,
    mut out: impl io::Write,
) -> io::Result<()> {
    write!(out, "P3\n{} {}\n255\n", width, height)?;

    let f = |v: f64| (255.0 * v.clamp(0.0, 1.0)).round() as u8;

    data.iter()
        .map(|c| (f(c.r), f(c.g), f(c.b)))
        .map(|(r, g, b)| writeln!(out, "{r} {g} {b}"))
        .collect::<Result<_, _>>()
}

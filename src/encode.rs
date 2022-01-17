use crate::Colour;
use std::io;

pub fn encode_ppm(
    data: &[Colour],
    width: u32,
    height: u32,
    mut out: impl io::Write,
) -> io::Result<()> {
    write!(out, "P3\n{} {}\n255\n", width, height)?;

    data.iter()
        .map(Colour::to_24bit_rgb)
        .map(|(r, g, b)| writeln!(out, "{r} {g} {b}"))
        .collect::<Result<_, _>>()
}

pub fn encode_webp(
    data: &[Colour],
    width: u32,
    height: u32,
    mut out: impl io::Write,
) -> io::Result<()> {
    let image = data
        .iter()
        .map(Colour::to_24bit_rgb)
        .flat_map(|(r, g, b)| [r, g, b])
        .collect::<Vec<u8>>();

    let enc = webp::Encoder::from_rgb(&image, width, height);

    out.write(&enc.encode_lossless()).map(|_| ())
}

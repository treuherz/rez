use std::io::Write;
use rez::{Colour, Ray, Vec3, Blend};

fn colour(r: &Ray) -> Colour {
    let t = (r.dir.unit().y + 1.0) / 2.0;
    Blend(Colour::new(1.0, 1.0, 1.0), Colour::new(0.5, 0.7, 1.0)).at(t)
}

fn main() -> std::io::Result<()> {
    let mut out = std::io::stdout();
    let mut err = std::io::stderr();

    const RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / RATIO) as u32;

    let vp_height = 2.0;
    let vp_width = RATIO * vp_height;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(vp_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, vp_height, 0.0);
    let lower_left = origin + Vec3::new(-vp_width / 2.0, -vp_height / 2.0, -focal_length);

    write!(out, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    for j in (0..IMAGE_HEIGHT).rev() {
        write!(err, "\rLines remaining: {:4}", j)?;
        err.flush()?;
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left + horizontal * u + vertical * v - origin,
            );
            let c = colour(&r);

            writeln!(out, "{}", c)?;
        }
    }

    write!(err, "\nDone\n")?;

    Ok(())
}

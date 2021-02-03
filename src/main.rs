use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut out = std::io::stdout();
    let mut err = std::io::stderr();

    const WIDTH: i32 = 256;
    const HEIGHT: i32 = 256;

    writeln!(out, "P3")?;
    writeln!(out, "{} {}", WIDTH, HEIGHT)?;
    writeln!(out, "255")?;

    for j in (0..HEIGHT).rev() {
        write!(err, "\rLines remaining: {:4}", j)?;
        err.flush()?;
        for i in 0..WIDTH {
            let (r, g, b) = (
                i as f64 / (WIDTH - 1) as f64,
                j as f64 / (HEIGHT - 1) as f64,
                0.25,
            );

            let coeff = 255.0;
            let (ir, ig, ib) = (
                (coeff * r) as i32,
                (coeff * g) as i32,
                (coeff * b) as i32,
            );

            writeln!(out, "{} {} {}", ir, ig, ib)?;
        }
    }

    write!(err, "\nDone\n")?;

    Ok(())
}

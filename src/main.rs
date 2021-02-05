#![feature(iterator_fold_self)]

use std::fs::File;
use std::io::Write;

use indicatif::{ProgressBar, ProgressStyle};
use itertools::iproduct;

use rez::{Blend, Camera, Collider, Colour, Ray, Sphere, Vec3};

fn colour<C: Collider>(r: Ray, world: C) -> Colour {
    if let Some(c) = world.collide(r, (0.0, f64::INFINITY)) {
        let n = c.normal;
        return Colour::new((n.x + 1.0) / 2.0, (n.y + 1.0) / 2.0, (n.z + 1.0) / 2.0);
    }

    let t = (r.dir.unit().y + 1.0) / 2.0;
    Blend(Colour::new(1, 1, 1), Colour::new(0.5, 0.7, 1.0)).at(t)
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let (stdout, mut lock, mut file);
    let out: &mut dyn Write = if args.len() >= 2 && &args[1] != "-" {
        file = File::create(&args[1])?;
        &mut file
    } else {
        stdout = std::io::stdout();
        lock = stdout.lock();
        &mut lock
    };

    // Image
    const RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / RATIO) as u64;
    const NUM_SAMPLES: u32 = 100;

    // World
    let world = {
        let mut world: Vec<Box<dyn Collider>> = Vec::new();
        world.push(Box::new(Sphere::new(Vec3::new(0, 0, -1), 0.5)));
        world.push(Box::new(Sphere::new(Vec3::new(0, -100.5, -1), 100.0)));
        world
    };

    // Camera
    let cam = Camera::new();

    // Render

    write!(out, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    let bar = ProgressBar::new(IMAGE_HEIGHT * IMAGE_WIDTH)
        .with_style(ProgressStyle::default_bar()
            .template("{percent:>3}% ▕{bar:40}▏ [{eta}/{elapsed}, {per_sec}]")
            .progress_chars("█▉▊▋▌▍▎▏ "));

    for (j, i) in iproduct!((0..IMAGE_HEIGHT).rev(), 0..IMAGE_WIDTH) {
        bar.inc(1);
        let mut samples = Vec::new();
        for _ in 0..NUM_SAMPLES {
            let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
            let r = cam.ray(u, v);

            samples.push(colour(r, &world));
        }
        let c = samples.iter()
            .copied()
            .fold_first(|a, b| (a + b))
            .unwrap() / NUM_SAMPLES;

        writeln!(out, "{}", c)?;
    }

    bar.finish_with_message("Done");

    Ok(())
}

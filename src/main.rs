#![feature(iterator_fold_self)]

use std::{fs::File, io};

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use itertools::{iproduct, Itertools};
use rand::random;

use rez::{Blend, Camera, Collider, Colour, Dielectric, Lambertian, Metal, Ray, Sphere, Vec3};

fn ray_colour<C: Collider>(r: Ray, world: C, depth: u32) -> Colour {
    if depth == 0 {
        return Colour::new(0, 0, 0);
    }

    if let Some(c) = world.collide(r, (0.001, f64::INFINITY)) {
        return if let Some((attenuation, scattered)) = c.scatter(r) {
            ray_colour(scattered, world, depth - 1).scale(attenuation)
        } else {
            Colour::new(0, 0, 0)
        };
    }

    let t = (r.dir.unit().y + 1.0) / 2.0;
    Blend(Colour::new(1, 1, 1), Colour::new(0.5, 0.7, 1.0)).at(t)
}

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let (stdout, mut lock, mut file);
    let out: &mut dyn io::Write = if args.len() >= 2 && &args[1] != "-" {
        file = File::create(&args[1])?;
        &mut file
    } else {
        stdout = io::stdout();
        lock = stdout.lock();
        &mut lock
    };

    // Image
    const RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / RATIO) as u64;
    const NUM_SAMPLES: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // World
    let mat_ground = Lambertian::new(Colour::new(0.8, 0.8, 0.0));
    let mat_mid = Lambertian::new(Colour::new(0.1, 0.2, 0.5));
    let mat_left = Dielectric::new(1.5);
    let mat_right = Metal::new(Colour::new(0.8, 0.6, 0.2), 0.0);

    let world: Vec<Box<dyn Collider>> = vec![
        Box::new(Sphere::new(Vec3::new(0, -100.5, -1), 100.0, &mat_ground)),
        Box::new(Sphere::new(Vec3::new(0, 0, -1), 0.5, &mat_mid)),
        Box::new(Sphere::new(Vec3::new(-1, 0, -1), 0.5, &mat_left)),
        Box::new(Sphere::new(Vec3::new(-1, 0, -1), -0.4, &mat_left)),
        Box::new(Sphere::new(Vec3::new(1, 0, -1), 0.5, &mat_right)),
    ];

    // Camera
    let cam = Camera::new();

    // Render

    write!(out, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    let bar = ProgressBar::new(IMAGE_HEIGHT * IMAGE_WIDTH).with_style(
        ProgressStyle::default_bar()
            .template("{percent:>3}% ▕{bar:40}▏ [{eta}/{elapsed}, {per_sec}]")
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );

    iproduct!((0..IMAGE_HEIGHT).rev(), 0..IMAGE_WIDTH)
        .progress_with(bar)
        .map(|(j, i)| {
            (0..NUM_SAMPLES)
                .map(|_| {
                    let u = (i as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.ray(u, v);

                    ray_colour(r, &world, MAX_DEPTH)
                })
                .sum::<Colour>()
        })
        .map(|c| writeln!(out, "{}", c))
        .fold_ok((), |_, _| ())
}

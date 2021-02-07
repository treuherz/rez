use std::sync::Arc;
use std::{fs::File, io};

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::{iproduct, Itertools};
use rand::{random, seq::SliceRandom, thread_rng, Rng};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use rez::{Blend, Camera, Collider, Colour, Dielectric, Lambertian, Metal, Ray, Sphere, Vec3};

fn ray_colour<C: Collider>(r: Ray, world: C, depth: u32) -> Colour {
    if depth == 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(c) = world.collide(r, (0.001, f64::INFINITY)) {
        return if let Some((attenuation, scattered)) = c.scatter(r) {
            ray_colour(scattered, world, depth - 1).scale(attenuation)
        } else {
            Colour::new(0.0, 0.0, 0.0)
        };
    }

    let t = (r.dir.unit().y + 1.0) / 2.0;
    Blend(Colour::WHITE, Colour::new(0.5, 0.7, 1.0)).at(t)
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
    const RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u64 = 100;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / RATIO) as u64;
    const NUM_SAMPLES: u32 = 200;
    const MAX_DEPTH: u32 = 50;

    // World
    let world = Arc::new(random_scene());

    // Camera
    let cam = Camera::builder()
        .origin(Vec3::new(13.0, 2.0, 3.0))
        .target(Vec3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .v_fov(f64::to_radians(20.0))
        .aspect_ratio(RATIO)
        .aperture(0.0)
        .focus_dist(10.0)
        .build()
        .unwrap();

    // Render

    let coords = {
        let mut coords: Vec<(u64, u64)> =
            iproduct!((0..IMAGE_HEIGHT).rev(), 0..IMAGE_WIDTH).collect();
        // Shuffle the coordinates. Will make ~0 difference to overall performance
        // but will make our progress bar move more evenly!
        coords.shuffle(&mut rand::thread_rng());
        coords
    };

    let mut pixels: Vec<((u64, u64), Colour)> = coords
        // .iter()
        .par_iter()
        .progress_with(progress_bar(IMAGE_HEIGHT * IMAGE_WIDTH))
        .map(|&(j, i)| {
            let col = (0..NUM_SAMPLES)
                .map(|_| {
                    let u = (i as f64 + random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.ray(u, v);

                    ray_colour(r, world.as_ref(), MAX_DEPTH)
                })
                .sum::<Colour>();
            ((i, j), col)
        })
        .collect::<Vec<((u64, u64), Colour)>>();

    pixels.par_sort_unstable_by_key(|((i, j), _)| (IMAGE_HEIGHT - j) * IMAGE_WIDTH + i);

    write!(out, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    pixels
        .iter()
        .map(|(_, c)| writeln!(out, "{}", c))
        .fold_ok((), |_, _| ())
}

fn random_scene() -> Vec<Box<dyn Collider + Send + Sync>> {
    let mut world: Vec<Box<dyn Collider + Send + Sync>> = Vec::new();

    // Ground
    let ground = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground,
    )));

    let glass = Arc::new(Dielectric::new(1.5));

    for (a, b) in iproduct!((-10..=10), (-10..=10)) {
        let centre = Vec3::new(
            a as f64 + 0.9 * random::<f64>(),
            0.2,
            b as f64 + 0.9 * random::<f64>(),
        );

        if (centre - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
            match random::<f64>() {
                r if (0.0..0.8).contains(&r) => {
                    let albedo = random();
                    let mat = Arc::new(Lambertian::new(albedo));
                    world.push(Box::new(Sphere::new(centre, 0.2, mat)));
                }
                r if (0.8..0.95).contains(&r) => {
                    let albedo = random();
                    let fuzz = thread_rng().gen_range(0.0..0.5);
                    let mat = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Box::new(Sphere::new(centre, 0.2, mat)));
                }
                r if (0.95..1.0).contains(&r) => {
                    world.push(Box::new(Sphere::new(centre, 0.2, glass.clone())))
                }
                _ => unreachable!(),
            };
        }
    }

    world.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1))),
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

fn progress_bar(len: u64) -> ProgressBar {
    let bar = ProgressBar::new(len);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{percent:>3}%▕{bar:40}▏ [{eta}/{elapsed}, {per_sec}]")
            .progress_chars("█▉▊▋▌▍▎▏ "),
    );
    bar.set_draw_delta(len / 1000);
    bar
}

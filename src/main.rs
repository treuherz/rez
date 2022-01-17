use std::{fs::File, io, sync::Arc};

use itertools::iproduct;
use rand::{random, thread_rng, Rng};

use rez::{
    encode_ppm, Camera, Collider, Colour, Dielectric, Lambertian, Metal, Raytracer, Sphere, Vec3,
};

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
    const IMAGE_HEIGHT: u32 = 100; //
    const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * RATIO) as u32;
    const NUM_SAMPLES: u32 = 100;
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

    let r = Raytracer::new(
        world,
        cam,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        NUM_SAMPLES,
        MAX_DEPTH,
    );

    let pixels = r.render();

    encode_ppm(&pixels, IMAGE_WIDTH, IMAGE_HEIGHT, out)
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

use std::io;
use std::sync::Arc;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::{iproduct, Itertools};
use rand::{random, seq::SliceRandom};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{Blend, Camera, Collider, Colour, Ray, Scene};

pub struct Raytracer {
    pub scene: Arc<Scene>,
    pub camera: Camera,

    pub width: u32,
    pub height: u32,

    pub samples_per_pixel: u32,
    pub bounce_depth: u32,
}

impl Raytracer {
    pub fn new(
        scene: Arc<Scene>,
        camera: Camera,
        width: u32,
        height: u32,
        samples_per_pixel: u32,
        bounce_depth: u32,
    ) -> Self {
        Raytracer {
            scene,
            camera,
            width,
            height,
            samples_per_pixel,
            bounce_depth,
        }
    }
}

impl Raytracer {
    pub fn render(&self, mut out: impl io::Write) -> io::Result<()> {
        let coords = {
            let mut coords: Vec<(u32, u32)> =
                iproduct!((0..self.height).rev(), 0..self.width).collect();
            // Shuffle the coordinates. Will make ~0 difference to overall performance
            // but will make our progress bar move more evenly!
            coords.shuffle(&mut rand::thread_rng());
            coords
        };

        let mut pixels: Vec<((u32, u32), Colour)> = coords
            .par_iter()
            .progress_with(progress_bar(self.height as u64 * self.width as u64))
            .map(|&(j, i)| {
                let col = (0..self.samples_per_pixel)
                    .map(|_| {
                        let u = (i as f64 + random::<f64>()) / (self.width - 1) as f64;
                        let v = (j as f64 + random::<f64>()) / (self.height - 1) as f64;
                        let r = self.camera.ray(u, v);

                        ray_colour(r, self.scene.as_ref(), self.bounce_depth)
                    })
                    .sum::<Colour>();
                ((i, j), col)
            })
            .collect();

        pixels.par_sort_unstable_by_key(|((i, j), _)| (self.height - j) * self.width + i);

        write!(out, "P3\n{} {}\n255\n", self.width, self.height)?;
        pixels
            .iter()
            .map(|(_, c)| writeln!(out, "{}", c))
            .fold_ok((), |_, _| ())
    }
}

fn ray_colour<C: Collider>(r: Ray, scene: C, depth: u32) -> Colour {
    if depth == 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    if let Some(c) = scene.collide(r, (0.001, f64::INFINITY)) {
        return if let Some((attenuation, scattered)) = c.scatter(r) {
            ray_colour(scattered, scene, depth - 1).scale(attenuation)
        } else {
            Colour::new(0.0, 0.0, 0.0)
        };
    }

    let t = (r.dir.unit().y + 1.0) / 2.0;
    Blend(Colour::WHITE, Colour::new(0.5, 0.7, 1.0)).at(t)
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

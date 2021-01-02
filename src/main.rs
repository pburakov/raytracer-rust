use std::f64::INFINITY;
use std::io;
use std::io::Write;

use crate::camera::Camera;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::random;
use crate::vector3::{Vector3 as Color, Vector3 as Point3, Vector3};

mod ray;
mod vector3;
mod hittable;
mod sphere;
mod hittable_list;
mod util;
mod camera;
mod color;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: u8 = 100;
const MAX_DEPTH: u8 = 50;

fn main() {
    // World
    let mut world = HittableList { objects: Vec::new() };
    world.add_sphere(Sphere { center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 });
    world.add_sphere(Sphere { center: Point3::new(0.0, -100.5, -1.0), radius: 100.0 });

    let camera = Camera::new();

    // Render
    io::stdout().write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)).unwrap();

    for y in (0..IMAGE_HEIGHT).rev() {
        io::stderr().write_fmt(format_args!("\rScanlines remaining: {} ", y)).unwrap();

        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + random()) / IMAGE_WIDTH as f64;
                let v = (y as f64 + random()) / IMAGE_HEIGHT as f64;

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    io::stderr().write_all(b"\nDone\n").unwrap();
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u8) -> Color {
    let rec = world.hit(r, 0.001, INFINITY);
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::zero();
    }
    if rec.is_some() {
        let target = rec.unwrap().p + Vector3::random_in_hemisphere(&rec.unwrap().normal);
        return 0.5 * ray_color(&Ray::new(rec.unwrap().p, target - rec.unwrap().p), world, depth - 1);
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

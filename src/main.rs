use std::io::{Stdout, Write};
use std::io;

use crate::ray::Ray;
use crate::vector3::Vector3;
use crate::vector3::Vector3 as Color;
use crate::vector3::Vector3 as Point3;

mod ray;
mod vector3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;


fn main() {
    let origin = Vector3::zero();
    let horizontal = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, FOCAL_LENGTH);

    io::stdout().write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)).unwrap();

    for y in (0..IMAGE_HEIGHT).rev() {
        io::stderr().write_fmt(format_args!("\rScanlines remaining: {}", y)).unwrap();

        for x in 0..IMAGE_WIDTH {
            let u = x as f64 / IMAGE_WIDTH as f64;
            let v = y as f64 / IMAGE_HEIGHT as f64;

            let r = Ray::new(&origin, &lower_left_corner + u * &horizontal + v * &vertical - &origin);
            let pixel_color = ray_color(&r);

            write_color(io::stdout(), pixel_color);
        }
    }

    io::stderr().write_all(b"\nDone\n").unwrap();
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - *center;
    let a = r.direction.dot(&r.direction);
    let b = 2.0 * oc.dot(&r.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn write_color(mut stdout: Stdout, color: Color) {
    stdout.write_fmt(format_args!("{} {} {}\n",
                                  (255.0 * color.x) as u8,
                                  (255.0 * color.y) as u8,
                                  (255.0 * color.z) as u8)).unwrap();
}

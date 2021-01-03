use std::f64::INFINITY;
use std::io;
use std::io::Write;

use crate::camera::Camera;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::random_f64;
use crate::vector3::{Vector3 as Color, Vector3 as Point3};

mod ray;
mod vector3;
mod hittable;
mod sphere;
mod util;
mod camera;
mod color;
mod material;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: u8 = 100;
const MAX_DEPTH: u8 = 50;

fn main() {
    // World
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground.clone())),
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center.clone())),
        Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())),
        Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, material_left.clone())),
        Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right.clone())),
    ];

    let camera = Camera::new();

    // Render
    io::stdout().write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)).unwrap();

    for y in (0..IMAGE_HEIGHT).rev() {
        io::stderr().write_fmt(format_args!("\rScanlines remaining: {} ", y)).unwrap();

        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + random_f64()) / IMAGE_WIDTH as f64;
                let v = (y as f64 + random_f64()) / IMAGE_HEIGHT as f64;

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    io::stderr().write_all(b"\nDone\n").unwrap();
}

fn ray_color(r: &Ray, world: &Vec<Box<dyn Hittable>>, depth: u8) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::zero();
    }
    let hit_rec = world.hit(r, 0.001, INFINITY);
    if hit_rec.is_some() {
        let hit = hit_rec.unwrap();
        let scatter_rec = hit.material.scatter(r, &hit.p, &hit.normal, hit.front_face);
        if scatter_rec.is_some() {
            let scatter = scatter_rec.unwrap();
            return scatter.attenuation * ray_color(&scatter.scattered_ray, world, depth - 1);
        } else {
            return Color::zero();
        }
    }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

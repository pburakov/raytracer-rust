use std::f64::INFINITY;
use std::io;
use std::io::Write;

use raytracer::camera::Camera;
use raytracer::color::write_color;
use raytracer::hittable::Hittable;
use raytracer::material::{Dielectric, Lambertian, Metal};
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::util::{random_f64, random_range};
use raytracer::vector3::{Vector3 as Color, Vector3 as Point3, Vector3};

// Image
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: u16 = 500;
const MAX_DEPTH: u8 = 50;

fn main() {
    // World

    let world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let v_up = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render
    io::stdout()
        .write_fmt(format_args!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT))
        .unwrap();

    for y in (0..IMAGE_HEIGHT).rev() {
        io::stderr()
            .write_fmt(format_args!("\rScanlines remaining: {} ", y))
            .unwrap();

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

fn random_scene() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::new_random() * Color::new_random();
                    let sphere_material = Lambertian::new(albedo);
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new_random_range(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)))
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.push(Box::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
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

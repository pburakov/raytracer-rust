use crate::ray::Ray;
use crate::util::degrees_to_radians;
use crate::vector3::{Vector3 as Point3, Vector3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, v_up: Vector3, v_fov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = degrees_to_radians(v_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = v_up.cross(&w).unit();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera { origin, lower_left_corner, horizontal, vertical, u, v, lens_radius }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vector3::new_random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}
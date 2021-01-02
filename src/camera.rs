use crate::ASPECT_RATIO;
use crate::ray::Ray;
use crate::vector3::{Vector3 as Point3, Vector3};

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

pub(crate) struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub(crate) fn new() -> Camera {
        let origin = Vector3::zero();
        let horizontal = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, FOCAL_LENGTH);

        Camera { origin: Point3::zero(), lower_left_corner, horizontal, vertical }
    }
    pub(crate) fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
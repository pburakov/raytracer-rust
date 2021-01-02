use crate::vector3::{Vector3, Vector3 as Point3};

// #[derive(Debug, PartialEq)]
pub struct Ray {
    pub(crate) origin: Point3,
    pub(crate) direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }
    pub(crate) fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
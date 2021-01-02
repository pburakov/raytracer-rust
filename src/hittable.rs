use crate::ray::Ray;
use crate::vector3::{Vector3 as Point3, Vector3};

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vector3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}

pub(crate) trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub(crate) fn from_normal(p: Point3, t: f64, r: &Ray, outward_normal: &Vector3) -> HitRecord {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal: Vector3;
        if front_face {
            normal = *outward_normal;
        } else {
            normal = -*outward_normal;
        }
        HitRecord { p, t, front_face, normal }
    }
}

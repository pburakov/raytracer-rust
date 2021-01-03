use crate::material::Material;
use crate::ray::Ray;
use crate::vector3::{Vector3 as Point3, Vector3};

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vector3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn from_normal<'a>(p: Point3, t: f64, r: &Ray, outward_normal: Vector3, material: &'a dyn Material) -> HitRecord<'a> {
        let front_face = r.direction.dot(&outward_normal) < 0.0;
        let mut normal = outward_normal;
        if !front_face {
            normal -= normal
        }
        HitRecord { p, t, front_face, normal, material }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut output: Option<HitRecord> = Option::None;
        let mut closest_so_far = t_max;

        for object in self {
            let maybe_hit = object.hit(r, t_min, closest_so_far);
            if maybe_hit.is_some() {
                closest_so_far = maybe_hit.unwrap().t;
                output = maybe_hit;
            }
        }

        output
    }
}

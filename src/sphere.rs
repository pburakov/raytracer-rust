use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vector3::Vector3 as Point3;

pub(crate) struct Sphere {
    pub(crate) center: Point3,
    pub(crate) radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return Option::None; }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return Option::None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Option::Some(HitRecord::from_normal(p, root, r, &outward_normal))
    }
}
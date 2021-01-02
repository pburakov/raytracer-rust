use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::sphere::Sphere;

pub(crate) struct HittableList {
    pub(crate) objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub(crate) fn add_sphere(&mut self, object: Sphere) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut output: Option<HitRecord> = Option::None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let maybe_hit = object.hit(r, t_min, closest_so_far);
            if maybe_hit.is_some() {
                closest_so_far = maybe_hit.unwrap().t;
                output = maybe_hit;
            }
        }

        output
    }
}

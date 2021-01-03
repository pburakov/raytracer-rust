use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector3::{Vector3 as Color, Vector3 as Point, Vector3};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered_ray: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, p: &Point, normal: &Vector3) -> Option<ScatterRecord>;
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, p: &Point, normal: &Vector3) -> Option<ScatterRecord> {
        let mut scatter_direction = normal + Vector3::new_random_unit_vector();
        // Catch degenerate scatter direction. If the random unit vector we generate is exactly
        // opposite the normal vector, the two will sum to zero, which will result in a zero
        // scatter direction vector.
        if scatter_direction.near_zero() {
            scatter_direction = *normal;
        }
        Option::Some(ScatterRecord { attenuation: self.albedo, scattered_ray: Ray::new(*p, scatter_direction) })
    }
}


#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Metal {
        Metal { albedo: color, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, p: &Point, normal: &Vector3) -> Option<ScatterRecord> {
        let reflected = r_in.direction.unit().reflect(normal);
        let scattered = Ray::new(*p, reflected + self.fuzz * Vector3::new_random_in_unit_sphere());
        if scattered.direction.dot(normal) > 0.0 {
            Option::Some(ScatterRecord { attenuation: self.albedo, scattered_ray: scattered })
        } else {
            Option::None
        }
    }
}
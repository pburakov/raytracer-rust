use crate::ray::Ray;
use crate::util::random_f64;
use crate::vector3::{Vector3 as Color, Vector3 as Point, Vector3};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered_ray: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, p: &Point, normal: &Vector3, front_face: bool) -> Option<ScatterRecord>;
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
    fn scatter(&self, _: &Ray, p: &Point, normal: &Vector3, _: bool) -> Option<ScatterRecord> {
        let attenuation = self.albedo;
        let mut scatter_direction = normal + Vector3::new_random_unit_vector();
        // Catch degenerate scatter direction. If the random unit vector we generate is exactly
        // opposite the normal vector, the two will sum to zero, which will result in a zero
        // scatter direction vector.
        if scatter_direction.near_zero() {
            scatter_direction = *normal;
        }
        let scattered_ray = Ray::new(*p, scatter_direction);
        Option::Some(ScatterRecord { attenuation, scattered_ray })
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
    fn scatter(&self, r_in: &Ray, p: &Point, normal: &Vector3, _: bool) -> Option<ScatterRecord> {
        let attenuation = self.albedo;
        let reflected = r_in.direction.unit().reflect(normal);
        let scattered_ray = Ray::new(*p, reflected + self.fuzz * Vector3::new_random_in_unit_sphere());
        if scattered_ray.direction.dot(normal) > 0.0 {
            Option::Some(ScatterRecord { attenuation, scattered_ray })
        } else {
            Option::None
        }
    }
}


#[derive(Clone)]
pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, p: &Vector3, normal: &Vector3, front_face: bool) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let mut refraction_ratio = self.ir;
        if front_face {
            refraction_ratio = 1.0 / self.ir;
        }

        let unit_direction = r_in.direction.unit();
        let cos_theta = (-unit_direction).dot(normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut direction = unit_direction.refract(normal, refraction_ratio);
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_f64() {
            direction = unit_direction.reflect(normal)
        }

        let scattered_ray = Ray::new(*p, direction);
        Option::Some(ScatterRecord { attenuation, scattered_ray })
    }
}


fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
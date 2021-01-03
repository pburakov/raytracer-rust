use auto_ops::*;

use crate::util::random_range;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }
    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
    pub fn new_random(min: f64, max: f64) -> Vector3 {
        Vector3 { x: random_range(min, max), y: random_range(min, max), z: random_range(min, max) }
    }
    pub fn new_random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::new_random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn new_random_unit_vector() -> Vector3 {
        Vector3::new_random_in_unit_sphere().unit()
    }
    pub fn new_random_in_hemisphere(normal: &Vector3) -> Vector3 {
        let in_unit_sphere = Vector3::new_random_in_unit_sphere();
        return if in_unit_sphere.dot(normal) > 0.0 { // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        };
    }
}

impl Vector3 {
    pub fn dot(&self, v: &Vector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn unit(&self) -> Vector3 {
        self / self.length()
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
    pub fn reflect(&self, normal: &Vector3) -> Vector3 {
        self - 2.0 * self.dot(normal) * normal
    }
}

impl_op!(+ |a: Vector3, b: Vector3| -> Vector3 { Vector3 {x: a.x+b.x, y: a.y+b.y, z: a.z+b.z} });
impl_op!(+ |a: Vector3, b: &Vector3| -> Vector3 { Vector3 {x: a.x+b.x, y: a.y+b.y, z: a.z+b.z} });
impl_op!(+ |a: &Vector3, b: Vector3| -> Vector3 { Vector3 {x: a.x+b.x, y: a.y+b.y, z: a.z+b.z} });
impl_op!(+ |a: &Vector3, b: &Vector3| -> Vector3 { Vector3 {x: a.x+b.x, y: a.y+b.y, z: a.z+b.z} });

impl_op!(+= |a: &mut Vector3, b: Vector3| { a.x+=b.x; a.y+=b.y; a.z+=b.z; });
impl_op!(+= |a: &mut Vector3, b: &Vector3| { a.x+=b.x; a.y+=b.y; a.z+=b.z; });
impl_op!(-= |a: &mut Vector3, b: Vector3| { a.x-=b.x; a.y-=b.y; a.z-=b.z; });
impl_op!(-= |a: &mut Vector3, b: &Vector3| { a.x-=b.x; a.y-=b.y; a.z-=b.z; });

impl_op!(- |a: Vector3| -> Vector3 { Vector3 {x: -a.x, y: -a.y, z: -a.z} });
impl_op!(- |a: &Vector3| -> Vector3 { Vector3 {x: -a.x, y: -a.y, z: -a.z} });

impl_op!(- |a: Vector3, b: Vector3| -> Vector3 { Vector3 {x: a.x-b.x, y: a.y-b.y, z: a.z-b.z} });
impl_op!(- |a: Vector3, b: &Vector3| -> Vector3 { Vector3 {x: a.x-b.x, y: a.y-b.y, z: a.z-b.z} });
impl_op!(- |a: &Vector3, b: Vector3| -> Vector3 { Vector3 {x: a.x-b.x, y: a.y-b.y, z: a.z-b.z} });
impl_op!(- |a: &Vector3, b: &Vector3| -> Vector3 { Vector3 {x: a.x-b.x, y: a.y-b.y, z: a.z-b.z} });

impl_op!(/ |a: Vector3, t: f64| -> Vector3 { Vector3 {x: a.x/t, y: a.y/t, z: a.z/t} });
impl_op!(/ |a: &Vector3, t: f64| -> Vector3 { Vector3 {x: a.x/t, y: a.y/t, z: a.z/t} });

impl_op!(* |a: Vector3, b: Vector3| -> Vector3 { Vector3 {x: a.x*b.x, y: a.y*b.y, z: a.z*b.z} });
impl_op!(* |a: Vector3, b: &Vector3| -> Vector3 { Vector3 {x: a.x*b.x, y: a.y*b.y, z: a.z*b.z} });
impl_op!(* |a: &Vector3, b: Vector3| -> Vector3 { Vector3 {x: a.x*b.x, y: a.y*b.y, z: a.z*b.z} });
impl_op!(* |a: &Vector3, b: &Vector3| -> Vector3 { Vector3 {x: a.x*b.x, y: a.y*b.y, z: a.z*b.z} });

impl_op!(* |t: f64, a: Vector3| -> Vector3 { Vector3 {x: a.x*t, y: a.y*t, z: a.z*t} });
impl_op!(* |t: f64, a: &Vector3| -> Vector3 { Vector3 {x: a.x*t, y: a.y*t, z: a.z*t} });

#[cfg(test)]
mod tests {
    use crate::vector3::Vector3;

    #[test]
    fn dot() {
        assert_eq!(26.0, Vector3::new(1.0, 2.0, 3.0).dot(&Vector3::new(3.0, 4.0, 5.0)));
    }

    #[test]
    fn length() {
        assert_eq!(1.0, Vector3::new(1.0, 0.0, 0.0).length());
        assert_eq!(2.0, Vector3::new(0.0, 2.0, 0.0).length());
        assert_eq!(3.0, Vector3::new(0.0, 0.0, 3.0).length());
    }

    #[test]
    fn length_squared() {
        assert_eq!(1.0, Vector3::new(1.0, 0.0, 0.0).length_squared());
        assert_eq!(4.0, Vector3::new(0.0, 2.0, 0.0).length_squared());
        assert_eq!(9.0, Vector3::new(0.0, 0.0, 3.0).length_squared());
    }

    #[test]
    fn unit() {
        assert_eq!(Vector3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0).unit());
        assert_eq!(Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 2.0, 0.0).unit());
        assert_eq!(Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 3.0).unit());
    }
}
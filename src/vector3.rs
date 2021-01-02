use auto_ops::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }
    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
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
        *self / self.length()
    }
}

impl_op!(+ |a: Vector3, b: Vector3| -> Vector3 { Vector3 {x: a.x+b.x, y: a.y+b.y, z: a.z+b.z} });
impl_op!(+ |a: &Vector3, b: Vector3| -> Vector3 { Vector3 {x: a.x+b.x, y: a.y+b.y, z: a.z+b.z} });

impl_op!(- |a: Vector3, b: Vector3| -> Vector3 { Vector3 {x: a.x-b.x, y: a.y-b.y, z: a.z-b.z} });
impl_op!(- |a: Vector3, b: &Vector3| -> Vector3 { Vector3 {x: a.x-b.x, y: a.y-b.y, z: a.z-b.z} });

impl_op!(/ |a: Vector3, t: f64| -> Vector3 { Vector3 {x: a.x/t, y: a.y/t, z: a.z/t} });

impl_op!(* |t: f64, a: Vector3| -> Vector3 { Vector3 {x: a.x*t, y: a.y*t, z: a.z*t} });
impl_op!(* |t: f64, a: &Vector3| -> Vector3 { Vector3 {x: a.x*t, y: a.y*t, z: a.z*t} });

impl_op!(- |a: Vector3| -> Vector3 { Vector3 {x:-a.x, y:-a.y, z:-a.z} });
impl_op!(- |a: &Vector3| -> Vector3 { Vector3 {x:-a.x, y:-a.y, z:-a.z} });

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
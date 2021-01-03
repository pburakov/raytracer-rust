use std::f64::consts::PI;

use rand::{Rng, thread_rng};

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    thread_rng().gen_range(0.0..1.0)
}

pub fn random_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    x
}

#[cfg(test)]
mod tests {
    use crate::util::{clamp, random_f64, random_range};

    #[test]
    fn random_bounds() {
        for _ in 0..1000 {
            let r = random_f64();
            assert_eq!(true, 0.0 <= r);
            assert_eq!(true, r < 1.0);
        }
    }

    #[test]
    fn random_range_bounds() {
        for _ in 0..1000 {
            let min = 0.0;
            let max = 1.0;
            let r = random_range(min, max);
            assert_eq!(true, min <= r);
            assert_eq!(true, r < max);
        }
    }

    #[test]
    fn test_clamp() {
        assert_eq!(0.0, clamp(-1.0, 0.0, 1.0));
        assert_eq!(1.0, clamp(2.0, 0.0, 1.0));
        assert_eq!(0.5, clamp(0.5, 0.0, 1.0));
    }
}

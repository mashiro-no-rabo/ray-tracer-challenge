use approx::{abs_diff_eq, AbsDiffEq};
use std::f64;

#[derive(Debug, PartialEq)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn is_point(&self) -> bool {
        abs_diff_eq!(self.w, &1.0)
    }

    fn is_vector(&self) -> bool {
        abs_diff_eq!(self.w, &0.0)
    }

    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    fn new_point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    fn add(&self, other: &Self) -> Self {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl AbsDiffEq<Tuple> for Tuple {
    type Epsilon = f64;

    fn default_epsilon() -> f64 {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon)
            && self.y.abs_diff_eq(&other.y, epsilon)
            && self.z.abs_diff_eq(&other.z, epsilon)
            && self.w.abs_diff_eq(&other.w, epsilon)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn tuple_as_point() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_abs_diff_eq!(a.x, 4.3);
        assert_abs_diff_eq!(a.y, -4.2);
        assert_abs_diff_eq!(a.z, 3.1);
        assert_abs_diff_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_as_vector() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert_abs_diff_eq!(a.x, 4.3);
        assert_abs_diff_eq!(a.y, -4.2);
        assert_abs_diff_eq!(a.z, 3.1);
        assert_abs_diff_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn create_point() {
        let p = Tuple::new_point(4.3, -4.2, 3.1);
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_abs_diff_eq!(p, a);
    }

    #[test]
    fn create_vector() {
        let p = Tuple::new_vector(4.3, -4.2, 3.1);
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert_abs_diff_eq!(p, a);
    }

    #[test]
    fn tuple_add() {
        let p = Tuple::new_point(3.0, -2.0, 5.0);
        let v = Tuple::new_vector(-2.0, 3.0, 1.0);
        let expected = Tuple::new(1.0, 1.0, 6.0, 1.0);

        assert_abs_diff_eq!(expected, p.add(&v));
    }
}

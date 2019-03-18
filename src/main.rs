use approx::AbsDiffEq;
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
        self.w.abs_diff_eq(&1.0 as &f64, f64::EPSILON)
    }

    fn is_vector(&self) -> bool {
        self.w.abs_diff_eq(&0.0 as &f64, f64::EPSILON)
    }

    fn new_point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0.0 }
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

        abs_diff_eq!(a.x, 4.3);
        abs_diff_eq!(a.y, -4.2);
        abs_diff_eq!(a.z, 3.1);
        abs_diff_eq!(a.w, 1.0);
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

        abs_diff_eq!(a.x, 4.3);
        abs_diff_eq!(a.y, -4.2);
        abs_diff_eq!(a.z, 3.1);
        abs_diff_eq!(a.w, 1.0);
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

        abs_diff_eq!(p, a);
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

        abs_diff_eq!(p, a);
    }
}

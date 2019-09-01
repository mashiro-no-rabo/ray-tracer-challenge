use super::operations::TupleOperations;
use std::f64;

#[derive(Debug, PartialEq)]
struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

#[allow(dead_code)]
impl Color {
    fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    fn mul(&self, other: &Self) -> Self {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl TupleOperations for Color {
    fn add(&self, other: &Self) -> Self {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }

    fn sub(&self, other: &Self) -> Self {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }

    fn muls(&self, scalar: f64) -> Self {
        Color {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    impl AbsDiffEq<Color> for Color {
        type Epsilon = f64;

        fn default_epsilon() -> f64 {
            // f64::EPSILON
            0.00001
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
            self.red.abs_diff_eq(&other.red, epsilon)
                && self.green.abs_diff_eq(&other.green, epsilon)
                && self.blue.abs_diff_eq(&other.blue, epsilon)
        }
    }

    #[test]
    fn create_color() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_abs_diff_eq!(c.red, -0.5);
        assert_abs_diff_eq!(c.green, 0.4);
        assert_abs_diff_eq!(c.blue, 1.7);
    }

    #[test]
    fn color_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(1.6, 0.7, 1.0);

        assert_abs_diff_eq!(expected, c1.add(&c2));
    }

    #[test]
    fn color_sub() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(0.2, 0.5, 0.5);

        assert_abs_diff_eq!(expected, c1.sub(&c2));
    }

    #[test]
    fn color_muls() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        let expected = Color::new(0.4, 0.6, 0.8);

        assert_abs_diff_eq!(expected, c1.muls(2.0));
    }

    #[test]
    fn color_hadamard_product() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let expected = Color::new(0.9, 0.2, 0.04);

        assert_abs_diff_eq!(expected, c1.mul(&c2));
    }
}

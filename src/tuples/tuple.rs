use approx::{abs_diff_eq, AbsDiffEq};
use std::f64;

use super::operations::TupleOperations;

#[derive(Debug, PartialEq)]
pub struct Tuple {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}

#[allow(dead_code)]
impl Tuple {
  pub fn is_point(&self) -> bool {
    abs_diff_eq!(self.w, &1.0)
  }

  pub fn is_vector(&self) -> bool {
    abs_diff_eq!(self.w, &0.0)
  }

  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
    Tuple { x, y, z, w }
  }

  pub fn new_point(x: f64, y: f64, z: f64) -> Self {
    Tuple { x, y, z, w: 1.0 }
  }

  pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
    Tuple { x, y, z, w: 0.0 }
  }

  pub fn neg(&self) -> Self {
    Tuple {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      w: -self.w,
    }
  }

  pub fn divs(&self, scalar: f64) -> Self {
    Tuple {
      x: self.x / scalar,
      y: self.y / scalar,
      z: self.z / scalar,
      w: self.w / scalar,
    }
  }

  pub fn magnitude(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
  }

  pub fn normalize(&self) -> Self {
    let m = self.magnitude();

    Tuple {
      x: self.x / m,
      y: self.y / m,
      z: self.z / m,
      w: self.w / m,
    }
  }

  pub fn dot(&self, other: &Self) -> f64 {
    (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
  }

  pub fn cross(&self, other: &Self) -> Self {
    let new_x = (self.y * other.z) - (self.z * other.y);
    let new_y = (self.z * other.x) - (self.x * other.z);
    let new_z = (self.x * other.y) - (self.y * other.x);
    Tuple::new_vector(new_x, new_y, new_z)
  }
}

impl TupleOperations for Tuple {
  fn add(&self, other: &Self) -> Self {
    Tuple {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
      w: self.w + other.w,
    }
  }

  fn sub(&self, other: &Self) -> Self {
    Tuple {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
      w: self.w - other.w,
    }
  }

  fn muls(&self, scalar: f64) -> Self {
    Tuple {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
      w: self.w * scalar,
    }
  }
}

impl AbsDiffEq<Tuple> for Tuple {
  type Epsilon = f64;

  fn default_epsilon() -> f64 {
    // f64::EPSILON
    0.00001
  }

  fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
    self.x.abs_diff_eq(&other.x, epsilon)
      && self.y.abs_diff_eq(&other.y, epsilon)
      && self.z.abs_diff_eq(&other.z, epsilon)
      && self.w.abs_diff_eq(&other.w, epsilon)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use approx::*;

  #[test]
  fn tuple_as_point() {
    let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

    assert_abs_diff_eq!(a.x, 4.3);
    assert_abs_diff_eq!(a.y, -4.2);
    assert_abs_diff_eq!(a.z, 3.1);
    assert_abs_diff_eq!(a.w, 1.0);
    assert!(a.is_point());
    assert!(!a.is_vector());
  }

  #[test]
  fn tuple_as_vector() {
    let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

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
    let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

    assert_abs_diff_eq!(p, a);
  }

  #[test]
  fn create_vector() {
    let p = Tuple::new_vector(4.3, -4.2, 3.1);
    let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

    assert_abs_diff_eq!(p, a);
  }

  #[test]
  fn tuple_add() {
    let p = Tuple::new_point(3.0, -2.0, 5.0);
    let v = Tuple::new_vector(-2.0, 3.0, 1.0);
    let expected = Tuple::new(1.0, 1.0, 6.0, 1.0);

    assert_abs_diff_eq!(expected, p.add(&v));
  }

  #[test]
  fn tuple_sub_pp() {
    let p1 = Tuple::new_point(3.0, 2.0, 1.0);
    let p2 = Tuple::new_point(5.0, 6.0, 7.0);
    let expected = Tuple::new_vector(-2.0, -4.0, -6.0);

    assert_abs_diff_eq!(expected, p1.sub(&p2));
  }

  #[test]
  fn tuple_sub_pv() {
    let p1 = Tuple::new_point(3.0, 2.0, 1.0);
    let p2 = Tuple::new_vector(5.0, 6.0, 7.0);
    let expected = Tuple::new_point(-2.0, -4.0, -6.0);

    assert_abs_diff_eq!(expected, p1.sub(&p2));
  }

  #[test]
  fn tuple_sub_vv() {
    let p1 = Tuple::new_vector(3.0, 2.0, 1.0);
    let p2 = Tuple::new_vector(5.0, 6.0, 7.0);
    let expected = Tuple::new_vector(-2.0, -4.0, -6.0);

    assert_abs_diff_eq!(expected, p1.sub(&p2));
  }

  #[test]
  fn tuple_neg_via_sub() {
    let zero = Tuple::new_vector(0.0, 0.0, 0.0);
    let v = Tuple::new_vector(1.0, -2.0, 3.0);
    let expected = Tuple::new_vector(-1.0, 2.0, -3.0);

    assert_abs_diff_eq!(expected, zero.sub(&v));
  }

  #[test]
  fn tuple_neg() {
    let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
    let expected = Tuple::new(-1.0, 2.0, -3.0, 4.0);

    assert_abs_diff_eq!(expected, t.neg());
  }

  #[test]
  fn tuple_mul_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

    let expected = Tuple::new(3.5, -7.0, 10.5, -14.0);
    assert_abs_diff_eq!(expected, a.muls(3.5));

    let expected2 = Tuple::new(0.5, -1.0, 1.5, -2.0);
    assert_abs_diff_eq!(expected2, a.muls(0.5));
  }

  #[test]
  fn tuple_div_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

    let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
    assert_abs_diff_eq!(expected, a.divs(2.0));
  }

  #[test]
  fn tuple_magnitude() {
    assert_abs_diff_eq!(1.0, Tuple::new_vector(1.0, 0.0, 0.0).magnitude());
    assert_abs_diff_eq!(1.0, Tuple::new_vector(0.0, 1.0, 0.0).magnitude());
    assert_abs_diff_eq!(1.0, Tuple::new_vector(0.0, 0.0, 1.0).magnitude());
    assert_abs_diff_eq!(14.0_f64.sqrt(), Tuple::new_vector(1.0, 2.0, 3.0).magnitude());
    assert_abs_diff_eq!(14.0_f64.sqrt(), Tuple::new_vector(-1.0, -2.0, -3.0).magnitude());
  }

  #[test]
  fn tuple_normalization() {
    let v = Tuple::new_vector(4.0, 0.0, 0.0);
    let expected = Tuple::new_vector(1.0, 0.0, 0.0);
    assert_abs_diff_eq!(expected, v.normalize());

    let v2 = Tuple::new_vector(1.0, 2.0, 3.0);
    let expected2 = Tuple::new_vector(0.26726, 0.53452, 0.80178);
    assert_abs_diff_eq!(expected2, v2.normalize());

    assert_abs_diff_eq!(1.0, v2.normalize().magnitude());
  }

  #[test]
  fn tuple_dot() {
    let a = Tuple::new_vector(1.0, 2.0, 3.0);
    let b = Tuple::new_vector(2.0, 3.0, 4.0);

    assert_abs_diff_eq!(20.0, a.dot(&b));
  }

  #[test]
  fn tuple_cross() {
    let a = Tuple::new_vector(1.0, 2.0, 3.0);
    let b = Tuple::new_vector(2.0, 3.0, 4.0);

    let expected = Tuple::new_vector(-1.0, 2.0, -1.0);
    let expected_rev = Tuple::new_vector(1.0, -2.0, 1.0);

    assert_abs_diff_eq!(expected, a.cross(&b));
    assert_abs_diff_eq!(expected_rev, b.cross(&a));
  }
}

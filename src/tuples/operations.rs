pub trait TupleOperations {
  fn add(&self, other: &Self) -> Self;
  fn sub(&self, other: &Self) -> Self;
  fn muls(&self, scalar: f64) -> Self;
}

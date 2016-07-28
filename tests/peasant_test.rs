extern crate quickcheck;
extern crate peasant;

#[cfg(test)]
mod test {
  use peasant::peasant;
  use quickcheck::quickcheck;

  #[test]
  fn test_peasant() {
    fn peasant_is_as_multiplication(a: usize, b:usize) -> bool {
      a.checked_mul(b) == peasant::peasant(a, b)
    }
    quickcheck(peasant_is_as_multiplication as fn(usize, usize) -> bool)
  }  
}
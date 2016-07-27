extern crate quickcheck;
extern crate peasant;

#[cfg(test)]
mod test {
  use peasant::peasant;
  use quickcheck::quickcheck;

  #[test]
  fn test_peasant() {
    fn peasant_is_as_multiplication(a: u32, b:u32) -> bool {
      a.checked_mul(b) == peasant::peasant(a, b)
    }
    quickcheck(peasant_is_as_multiplication as fn(u32, u32) -> bool)
  }  
}
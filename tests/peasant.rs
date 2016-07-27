extern crate quickcheck;

use quickcheck::quickcheck;


fn peasant(a: u32, b: u32) -> Option<u32> {
  fn even(value: u32) -> bool {
    value & 1 == 0
  }

  fn run(a: u32, b: u32) -> Option<u32> {
    match (a, b) {
      (0, _) | (_, 0) => Some(0),
      (1, _)          => Some(b),
      (_, _)          => 
        match (a.checked_div(2), b.checked_add(b)) {
          (Some(newa), Some(newb)) => {
                        let odd_balance = if even(a) { 0 } else { b };
                        run(newa, newb).map(|result| result + odd_balance)
                      },
          (_, _)  => None
        }
    }
  }

  run(a, b)
}

#[test]
fn test_peasant() {
  fn peasant_is_as_multiplication(a: u32, b:u32) -> bool {
    a.checked_mul(b) == peasant(a, b)
  }
  quickcheck(peasant_is_as_multiplication as fn(u32, u32) -> bool)
}
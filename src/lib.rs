fn main() {
    println!("Hello, world!");
}

pub mod peasant {
  pub fn peasant(a: u32, b: u32) -> Option<u32> {
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
}
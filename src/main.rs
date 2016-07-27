use std::env;
extern crate peasant;

fn get_u32(index: usize) -> Option<u32>{
  env::args().nth(index).and_then(|v| v.parse::<u32>().ok())
}

fn main() {
  match (get_u32(1), get_u32(2))  {
    (Some(first), Some(second)) => {
        use peasant::peasant;
        match peasant::peasant(first,second) {
          Some(result) => println!("The result of peasanting {} and {} is {}", 
                              first.to_string(), second.to_string(), result.to_string()),
          None         => println!("Ну не смогла")
        }
    }
    _               => println!("Please provide 2 valid numbers to peasant them together, max value is {}", std::u32::MAX.to_string())
  }
}
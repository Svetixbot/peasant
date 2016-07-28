use std::env;
extern crate peasant;

fn get_number(index: usize) -> Option<usize>{
  env::args().nth(index).and_then(|v| v.parse::<usize>().ok())
}

fn main() {
  match (get_number(1), get_number(2))  {
    (Some(first), Some(second)) => {
        use peasant::peasant;
        match peasant::peasant(first,second) {
          Some(result) => println!("The result of peasanting {} and {} is {}", 
                              first, second, result),
          None         => println!("Ну не смогла")
        }
    }
    _               => println!("Please provide 2 valid numbers to peasant them together, max value is {}", std::usize::MAX)
  }
}
# Peasant Multiplication

The algorithm can be found [here](http://www.cut-the-knot.org/Curriculum/Algebra/PeasantMultiplication.shtml)
I don't really know how optimal this is and what is the right solution, who cares.

# Tech choice

 - `Rust` language, cause I was inspired by [Dave Herman's talk at Curry on](https://www.youtube.com/watch?v=9OHcJzJQ2Nk) and wanted to see what the heck is it about.

 - [`Quickcheck`](https://github.com/BurntSushi/quickcheck) implementation in Rust, cause property-based testing was such a good fit for this problem.

# How to run

Make sure to install rust and cargo first
```sh
brew install rust
```

Run it with cargo by providing numbers as command line arguments
```sh
cargo run 85 18
```

Test it with cargo
```sh
cargo test
```

# Things I like about this solution:

### Property-based testing. In fact, TDD with property-based testing. Having 1 single test, which coveres all the edge cases.

```rust
#[test]
fn test_peasant() {
  fn peasant_is_as_multiplication(a: usize, b:usize) -> bool {
    a.checked_mul(b) == peasant(a, b)
  }
  quickcheck(peasant_is_as_multiplication as fn(usize, usize) -> bool)
}
```

### Overflow and underflow safe solution.

Obviosly multiplication of usize numbers can be more than usize. For some reason Rust is using `Option` monad for wrapping the computation result. I would expect something like `Either`, but it is still better than exception in your face.

```rust
fn peasant(a: usize, b: usize) -> Option<usize> 
```

### No mutation. 
 But yeh, there is no tail recursion implemented in Rust yet, so it can fail with stack overflow. There is a [workaround](https://crates.io/crates/stacker).

# Struggles:

### `Parametric polymorphism` in Rust?

I want to have this:
 ```rust
fn peasant<N: Num>(a: N, b: N) -> Option<N> {
```
instead of this:
 ```rust
fn peasant(a: usize, b: usize) -> Option<usize> {
```

And it is quite possible, it would look like below. But, it would not compile, cause One and Zero are `unstabled features` of Rust, which can't be enabled on a stable compiler version. And on top of it, making your code to be generic in rust just sucks the whole joy out of the process. #nofuckingwayimdoingit

```rust
use std::ops::BitAnd;
use std::cmp::PartialEq;
use std::num::{One, Zero};
  
fn even<T>(value: T) -> bool 
  where T: BitAnd<T, Output = T> + PartialEq + One + Zero {
  value & T::one() == T::zero()
}
```

However, if the above thing is possible, what about quickcheck test? I want this quickcheck thing to be generic as well:

```rust
quickcheck(peasant_is_as_multiplication as fn<N: Num>(N, N) -> bool)
```

### Stucture the code
What is idiomatic way of structuring code and unit tests? I put tests and code into seprate files. I have created 2 modules: tests and peasant. The import of peasant module in test file and main file are just `weird`. Am I doing it right?

### Also no list comprehension?
It is such a nice syntax sugar for composing things:

Imagine this:
```rust
  env::args().nth(index).and_then(|v| v.parse::<usize>().ok())
```
would look like this:
```rust
for {
  number_from_cmd <- env::args().nth(index)
  parsed          <- number_from_cmd.parse::<usize>().ok()
} yield parsed
```
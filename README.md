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
  fn peasant_is_as_multiplication(a: u32, b:u32) -> bool {
    a.checked_mul(b) == peasant(a, b)
  }
  quickcheck(peasant_is_as_multiplication as fn(u32, u32) -> bool)
}
```

### Overflow and underflow safe solution.

Obviosly multiplication of u32 numbers can be more than u32. For some reason Rust is using `Option` monad for wrapping the computation result. I would expect something like `Either`, but it is still better than exception in your face.

```rust
fn peasant(a: u32, b: u32) -> Option<u32> 
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
fn peasant(a: u32, b: u32) -> Option<u32> {
```

And if above thing is possible, what about quickcheck test? I want this quickcheck thing to be generic as well:

```rust
quickcheck(peasant_is_as_multiplication as fn<N: Num>(N, N) -> bool)
```

### Stucture the code
What is idiomatic way of structuring code and unit tests? I put tests and code into seprate files. I have created 2 modules: tests and peasant. The import of peasant module in test file and main file are just `weird`. Am I doing it right?
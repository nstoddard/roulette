**Note: the `rand` crate now has [built-in-support for the Alias method](https://rust-random.github.io/rand/rand/distributions/weighted/alias_method/index.html), so this library is no longer needed.**

A Rust implementation of roulette wheel selection using the Alias Method.
This can be used to simulate a loaded die and similar situations.

Initialization takes O(n) time; choosing a random element takes O(1) time.
This is far faster than naive algorithms (the most common of which is
commonly known as 'roulette wheel selection'). For an in-depth explanation
of the algorithm, see http://www.keithschwarz.com/darts-dice-coins/.

This code was translated from
http://www.keithschwarz.com/interesting/code/?dir=alias-method.

# Example

Run example with `cargo run --example simple`

```rust
extern crate rand;
extern crate roulette;

use roulette::Roulette;

fn main() {
  let mut rng = rand::thread_rng();
  let roulette = Roulette::new(vec![
      ('a', 1.0), ('b', 1.0), ('c', 0.5), ('d', 0.0)]);
  for _ in 0..10 {
      let rand = roulette.sample(&mut rng);
      println!("{}", rand);
  }
}
```

In this example, `rand` will be 'a' with 40% probability, 'b' with 40% probability, and 'c' with 20% probability.

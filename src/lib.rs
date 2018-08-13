//! A Rust implementation of roulette wheel selection using the
//! Alias Method. This can be used to simulate a loaded die and similar
//! situations.
//!
//! Initialization takes O(n) time; choosing a random element takes O(1) time.
//! This is far faster than naive algorithms (the most common of which is
//! commonly known as 'roulette wheel selection'). For an in-depth explanation
//! of the algorithm, see http://www.keithschwarz.com/darts-dice-coins/.
//!
//! This code was translated from
//! http://www.keithschwarz.com/interesting/code/?dir=alias-method.
//!
//! # Example
//!
//! ```rust
//! extern crate rand;
//! extern crate roulette;
//!
//! use roulette::Roulette;
//!
//! fn main() {
//!     let mut rng = rand::thread_rng();
//!     let roulette = Roulette::new(vec![('a', 1.0), ('b', 1.0), ('c', 0.5), ('d', 0.0)]);
//!     for _ in 0..10 {
//!         let rand = roulette.sample(&mut rng);
//!         println!("{}", rand);
//!     }
//! }
//! ```

extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

/// An efficient implementation of roulette wheel selection. This can be
/// used to simulate a loaded die.
pub struct Roulette<T> {
    probabilities: Vec<T>,
    alias: Vec<usize>,
    probability: Vec<f64>,
    range: Uniform<usize>,
}

impl<T> Roulette<T> {
    /// Creates a `Roulette` with the given probabilities for a set of elements.
    /// Note that the probabilities don't have to sum to 1;
    /// they will be normalized automatically.
    ///
    /// Panics if the probabilities are all zero or if any are negative.
    pub fn new(probabilities: Vec<(T, f64)>) -> Roulette<T> {
        let len = probabilities.len();
        let range = Uniform::from(0..len);

        let sum: f64 = probabilities.iter().map(|x| x.1).sum();

        for prob in &probabilities {
            if prob.1 < 0.0 {
                panic!("Invalid probability in Roulette: must not be negative");
            }
        }
        assert!(sum != 0.0, "Probabilities in Roulette must not all be zero");

        let inv_sum = 1.0 / sum;
        let mut prob: Vec<_> = probabilities.iter().map(|x| x.1 * inv_sum).collect();

        let average = 1.0 / len as f64;
        let mut small = Vec::new();
        let mut large = Vec::new();
        for (i, prob) in prob.iter().enumerate().take(len) {
            if *prob >= average {
                large.push(i);
            } else {
                small.push(i);
            }
        }

        let mut alias = vec![0; len];
        let mut probability = vec![0.0; len];

        while !small.is_empty() && !large.is_empty() {
            let less = small.pop().unwrap();
            let more = large.pop().unwrap();
            probability[less] = prob[less] * len as f64;
            alias[less] = more;
            prob[more] = (prob[more] + prob[less]) - average;
            if prob[more] >= average {
                large.push(more);
            } else {
                small.push(more);
            }
        }

        while !small.is_empty() {
            probability[small.pop().unwrap()] = 1.0;
        }
        while !large.is_empty() {
            probability[large.pop().unwrap()] = 1.0;
        }

        Roulette {
            probabilities: probabilities.into_iter().map(|x| x.0).collect(),
            alias,
            probability,
            range,
        }
    }

    /// Returns a random element; each element's chance of being returned
    /// is proportional to the probability specified in the parameter
    /// to `Roulette::new`.
    pub fn sample<R: Rng>(&self, rng: &mut R) -> &T {
        let column = self.range.sample(rng);
        let coin = rng.gen::<f64>() < self.probability[column];
        &self.probabilities[if coin { column } else { self.alias[column] }]
    }
}

#[cfg(test)]
mod tests {
    // TODO: is there a way to test the distribution returned from `sample` in a
    // deterministic way?

    use super::*;

    #[test]
    fn most_entries_zero() {
        let roulette = Roulette::new(vec![('a', 0.0), ('b', 1.0), ('c', 0.0), ('d', 0.0)]);
        for _ in 0..10 {
            assert_eq!(&'b', roulette.sample(&mut rand::thread_rng()));
        }
    }

    #[test]
    #[should_panic]
    fn all_entries_zero() {
        Roulette::new(vec![('a', 0.0), ('b', 0.0), ('c', 0.0), ('d', 0.0)]);
    }

    #[test]
    #[should_panic]
    fn negative_entry() {
        Roulette::new(vec![('a', 0.0), ('b', 1.0), ('c', 0.0), ('d', -0.5)]);
    }
}

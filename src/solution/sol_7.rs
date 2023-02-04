// Project Euler: Problem 7
use std::fmt::Display;

/// Solution to Problem 7
///
///    By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
///   that the 6th prime is 13.
///
///   What is the 10 001st prime number?
///
/// NOTE: Auto-generated, don't change signature
pub fn solution() -> impl Display {
    let primes = crate::common::primes(1_000_000);
    primes
        .get(10_000)
        .expect("expected item to be below 1M")
        .clone()
}

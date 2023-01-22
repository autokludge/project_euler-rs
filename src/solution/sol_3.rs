// Project Euler: Problem 3
use std::fmt::Display;

/// Solution to Problem 3
///
///    The prime factors of 13195 are 5, 7, 13 and 29.
///
///   What is the largest prime factor of the number 600851475143 ?
///
/// NOTE: Auto-generated, don't change signature
pub fn solution() -> impl Display {
    let n: usize = 600851475143;
    let sqrt_n = (n as f64).sqrt() as usize;
    let primes = crate::common::primes(sqrt_n);
    primes
        .into_iter()
        .filter(|&p| n % p == 0)
        .max()
        .expect("primes should have elements")
}

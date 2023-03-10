// Project Euler: Problem 1
use std::fmt::Display;

/// Solution to Problem 1
///
///    If we list all the natural numbers below 10 that are multiples of 3 or 5,
///   we get 3, 5, 6 and 9. The sum of these multiples is 23.
///
///   Find the sum of all the multiples of 3 or 5 below 1000.
///
/// NOTE: Auto-generated, don't change signature
pub fn solution() -> impl Display {
    let x: u64 = (0..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum();
    x
}

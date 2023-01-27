// Project Euler: Problem 5
use std::fmt::Display;

use itertools::Itertools;

/// Solution to Problem 5
///
///    2520 is the smallest number that can be divided by each of the numbers
///   from 1 to 10 without any remainder.
///
///   What is the smallest positive number that is evenly divisible by all of
///   the numbers from 1 to 20?
///
/// NOTE: Auto-generated, don't change signature
pub fn solution() -> impl Display {
    lowest_common_multiple((1..=20).collect_vec())
}

fn lowest_common_multiple(ns: Vec<usize>) -> usize {
    ns.into_iter().fold(1, |a, e| lcm(a, e))
}

fn gcd(a: usize, b: usize) -> usize {
    let mut ab = (a, b);
    while ab.1 != 0 {
        ab = (ab.1, ab.0 % ab.1)
    }
    ab.0
}
fn lcm(a: usize, b: usize) -> usize {
    let g = gcd(a, b);
    (a / g) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_lcm_1_to_10() {
        assert_eq!(lowest_common_multiple((1..=10).collect_vec()), 2520);
    }
}

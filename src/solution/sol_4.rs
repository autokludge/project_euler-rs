// Project Euler: Problem 4
use itertools::Itertools;
use std::fmt::Display;

/// Solution to Problem 4
///
///    A palindromic number reads the same both ways. The largest palindrome made
///   from the product of two 2-digit numbers is 9009 = 91 × 99.
///
///   Find the largest palindrome made from the product of two 3-digit numbers.
///
/// NOTE: Auto-generated, don't change signature
pub fn solution() -> impl Display {
    let limit = 999;
    (0..=limit)
        .into_iter()
        .combinations(2)
        .map(|v| v.iter().product())
        .filter(|p| is_palindrome(p))
        .max()
        .unwrap()
}

fn is_palindrome(n: &u64) -> bool {
    let n_string = n.to_string();
    let n_string_reversed: String = n_string.chars().rev().collect();
    n_string == n_string_reversed
}

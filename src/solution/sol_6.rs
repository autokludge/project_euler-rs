// Project Euler: Problem 6
use std::fmt::Display;

/// Solution to Problem 6
///
///    The sum of the squares of the first ten natural numbers is,
///
///                          1^2 + 2^2 + ... + 10^2 = 385
///
///   The square of the sum of the first ten natural numbers is,
///
///                       (1 + 2 + ... + 10)^2 = 55^2 = 3025
///
///   Hence the difference between the sum of the squares of the first ten
///   natural numbers and the square of the sum is 3025 − 385 = 2640.
///
///   Find the difference between the sum of the squares of the first one
///   hundred natural numbers and the square of the sum.
///
/// NOTE: Auto-generated, don't change signature
pub fn solution() -> impl Display {
    let n: i32 = 100;
    let sum = n * (n + 1) / 2; // Gauss progression formula
    let square_sum = sum * sum;
    let sum_squares = (2 * n + 1) * (n + 1) * n / 6;
    sum_squares.abs_diff(square_sum)
}

// pub fn solution_1() -> impl Display {
//     let n = 100;
//     let sum_squares: u64 = (1..=n).map(|x| x * x).sum();
//     let square_sums: u64 = (1..=n).sum::<u64>().pow(2);
//     sum_squares.abs_diff(square_sums)
// }

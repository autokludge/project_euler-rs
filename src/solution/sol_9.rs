// Project Euler: Problem 9
use itertools::Itertools;
use std::fmt::Display;

/// Solution to Problem 9
///
///    A Pythagorean triplet is a set of three natural numbers, a < b < c, for
///   which,
///
///                                a^2 + b^2 = c^2
///
///   For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
///
///   There exists exactly one Pythagorean triplet for which a + b + c = 1000.
///   Find the product abc.
///
/// NOTE: Auto-generated, don't change signature
pub fn solution() -> impl Display {
    // lazy iterator with early return on solution, so no need to tune range upper bound
    for (a, b, c) in (0..1000u32).tuple_combinations() {
        // no need to check if a < b < c, as tuple_combinations produces a sorted tuple
        if a + b + c != 1000 {
            continue;
        }
        if a.pow(2) + b.pow(2) == c.pow(2) {
            return a * b * c;
        }
    }
    0
}

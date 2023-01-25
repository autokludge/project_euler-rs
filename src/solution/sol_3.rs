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
    second_attempt(600851475143)
}

// rather than generating primes, we will rely on other properties
// for any n there can be at most 1 prime factor over √n
// lets reduce n by factor as we go (n=n/3; n%7==0) rather than checking n%7==0
// handle 2 seperately, then just check odd numbers
fn second_attempt(n: u64) -> impl Display {
    let mut n: u64 = n;
    let mut lastfactor = 1;
    lastfactor = remove_factor(&mut n, 2, lastfactor);

    let mut factor = 3;
    let mut factor_sqrt_limit = (n as f64).sqrt() as u64 + 1;
    while n > 1 && factor < factor_sqrt_limit {
        lastfactor = remove_factor(&mut n, factor, lastfactor);
        factor_sqrt_limit = (n as f64).sqrt() as u64 + 1;
        factor += 2;
    }
    if n == 1 {
        lastfactor
    } else {
        n
    }
}

fn remove_factor(n: &mut u64, f: u64, lf: u64) -> u64 {
    let mut lf = lf;
    while *n % f == 0 {
        *n /= f;
        lf = f;
    }
    lf
}

// *** this worked, but is problematic ***
// we weren't guarranteed the largest prime factor will be below sqrt(n)
// generating all primes up to n takes too much memory with naive prime generator
// iterator filter checks all primes before producing max value.
// do we need primes first, or is it ok to just check all odd numbers (and 2)?
// *****
// fn first_attempt() -> impl Display {
//     let n: usize = 600851475143;
//     let sqrt_n = (n as f64).sqrt() as usize;
//     let primes = crate::common::primes(sqrt_n);
//     primes
//         .into_iter()
//         .filter(|&p| n % p == 0)
//         .max()
//         .expect("primes should have elements")
// }

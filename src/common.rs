#![allow(dead_code)]

pub fn primes(n: usize) -> Vec<usize> {
    // sieve of eratothenes ?
    // sieve of atkin ?
    // sieve of zakiya
    primes_soe_1(n)
}

fn primes_soe_1(n: usize) -> Vec<usize> {
    let mut sieve = vec![true; n];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..(n as f64).sqrt() as usize {
        if !sieve[i] {
            continue;
        }
        for ic in (i * i..n).step_by(i) {
            sieve[ic] = false;
        }
    }

    //
    sieve
        .iter()
        .enumerate()
        .filter(|(_, &v)| v)
        .map(|(k, _)| k)
        .collect()
}


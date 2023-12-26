use super::common::primes;
use itertools::{partition, Itertools};

pub fn problem() {
    let b_options = primes(1000)
        .into_iter()
        .map(|x| x as i128);
    let primes = primes(1_000_000);

    let answer_triple = (-999..1000).step_by(2)
        .cartesian_product(b_options)
        .map(|(a, b)| (a, b, consecutive_prime_count(a, b, &primes)))
        .max_by_key(|&(_, _, count)| count)
        .unwrap();
    let answer = answer_triple.0 * answer_triple.1;
    println!("answer = {}", answer);
}

fn f(n: i128, a: i128, b: i128) -> i128 {
    n * n + a * n + b
}

/// assumes f(1) is prime
fn consecutive_prime_count(a: i128, b: i128, primes: &Vec<u64>) -> usize {
    let mut count = 1;
    let mut x = f(count, a, b);
    if x > 1_000_000 {
        panic!("x is too large");
    }
    while primes.binary_search(&(x as u64)).is_ok() {
        count += 1;
        x = f(count, a, b);
        if x < 0 {
            return count as usize;
        }
    }
    count as usize
}

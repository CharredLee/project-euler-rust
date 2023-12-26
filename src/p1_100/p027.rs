use super::common::primes;
use itertools::{partition, Itertools};

pub fn problem() {
    let b_options = primes(1000)
        .into_iter()
        .map(|x| x as i128)
        .collect_vec();
    let primes = primes(1_000_000);
    let mut max = 0;
    let mut max_a = 0;
    let mut max_b = 0;
    // a must be odd
    for a in (-999..1000).step_by(2) {
        // b must be prime, and f(39)=39^2+39a+b must be prime, 
        // hence positive, so b > 39^2+39a
        for &b in b_options.iter().skip_while(|&&b| b <= 39 * 39 + 39 * a) {
            let count = consecutive_prime_count(a, b, &primes);
            if count > max {
                max = count;
                max_a = a;
                max_b = b;
            }
        }
    }
    println!("answer: {}", max_a * max_b);
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

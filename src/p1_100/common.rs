use divisors::get_divisors;
use itertools::Itertools;
use num::{BigInt, Num};
use memoize::memoize;

pub const PHI: f64 = 1.618_033_988_749_895;

pub fn triangle<T: Num + Copy>(n: T) -> T {
    // the nth triangular number.
    n * (n + T::one()) / (T::one() + T::one())
}

pub fn multiple_count<T: Num + Copy>(n: T, limit: T) -> T {
    // the number of multiples of n less than limit.
    limit / n
}

pub fn multiple_sum<T: Num + Copy>(n: T, limit: T) -> T {
    // the sum of all multiples of n less than limit.
    n * triangle(multiple_count(n, limit))
}

pub fn square_sum<T: Num + Copy>(n: T) -> T {
    // the sum of squares through n: 1^2 + ... + n^2
    n * (n + T::one()) * ((T::one() + T::one()) * n + T::one())
        / ((T::one() + T::one() + T::one()) * (T::one() + T::one()))
}

pub fn euler_method<T: Num + Copy + PartialOrd>(m: T, n: T) -> Vec<T> {
    assert!(m > n);
    vec![m * m - n * n, (T::one() + T::one()) * m * n, m * m + n * n]
}

pub fn collatz<T: Num + Copy>(n: T) -> T {
    if n % (T::one() + T::one()) == T::zero() {
        return n / (T::one() + T::one());
    }
    (T::one() + T::one() + T::one()) * n + T::one()
}

pub fn factorial<T: Num + Copy>(n: T) -> T {
    match n == T::zero() {
        true => T::one(),
        false => n * factorial(n - T::one()),
    }
}

/// Returns the sum of all divisors of n, other than n.
pub fn sum_proper_divisors(n: u64) -> u64 {
    get_divisors(n).iter().sum::<u64>()
}

fn fibonacci_helper<T: Num + Copy>(n: T) -> (T, T) {
    match n == T::zero() {
        true => (T::zero(), T::one()),
        false => {
            let (x, y) = fibonacci_helper(n / (T::one() + T::one()));
            let (z, w) = (x * (y + y - x), x * x + y * y);
            match n % (T::one() + T::one()) == T::zero() {
                true => (z, w),
                false => (w, z + w),
            }
        }
    }
}

pub fn fib<T: Num + Copy>(n: T) -> T {
    fibonacci_helper(n).0
}

fn big_fibonacci_helper(n: &BigInt) -> (BigInt, BigInt) {
    match n == &BigInt::from(0) {
        true => (BigInt::from(0), BigInt::from(1)),
        false => {
            let (x, y) = big_fibonacci_helper(&(n / 2));
            let (z, w) = (&x * (&y + &y - &x), &x * &x + &y * &y);
            match n % 2 == BigInt::from(0) {
                true => (z, w),
                false => (w.clone(), z + w),
            }
        }
    }
}

pub fn big_fib(n: &BigInt) -> BigInt {
    big_fibonacci_helper(n).0
}


#[memoize]
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    } else if n == 2 || n == 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    } else {
        for i in (5..).step_by(6).take_while(|&i| i * i <= n) {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
        }
        return true;
    }
}


pub fn primes(limit: usize) -> Vec<u64> {
    let mut out = (0..limit as u64).collect_vec();
    out[0] = 0;
    out[1] = 0;
    for i in 2..limit {
        if out[i as usize] != 0 {
            for j in (i * i..limit).step_by(i) {
                out[j] = 0;
            }
        }
    }
    out.into_iter()
       .filter(|&x| x != 0)
       .collect::<Vec<u64>>()
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checking_is_prime() {
        // write tests for is_prime here
        for i in 0..100 {
            let primality = match i {
                2 |  3 |  5 |  7 | 11 | 13 | 17 | 19 | 23 
                  | 29 | 31 | 37 | 41 | 43 | 47 | 53 | 59 
                  | 61 | 67 | 71 | 73 | 79 | 83 | 89 | 97 
                  => true,
                _ => false,
            };
            assert_eq!(is_prime(i), primality);
        }
    }
}
 */
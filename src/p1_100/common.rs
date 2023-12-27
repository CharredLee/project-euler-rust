use divisors::get_divisors;
use itertools::Itertools;
use num::{BigInt, Num};
use memoize::memoize;


pub const PHI: f64 = 1.618_033_988_749_895;

pub fn triangle<T: Num + Copy>(n: T) -> T {
    // the nth triangular number.
    n * (n + T::one()) / (T::one() + T::one())
}

pub fn sum_of_squares<T: Num + Copy>(n: T) -> T {
    // the sum of squares 1^2 + ... + n^2.
    let one = T::one();
    let two = one + one;
    let three = two + one;
    n * (n + one) * (two * n + one) / (two * three)
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
        false
    } else if n == 2 || n == 3 {
        true
    } else if n % 2 == 0 || n % 3 == 0 {
        false
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
        if out[i] != 0 {
            for j in (i * i..limit).step_by(i) {
                out[j] = 0;
            }
        }
    }
    out.into_iter()
       .filter(|&x| x != 0)
       .collect::<Vec<u64>>()
}

#[memoize]
pub fn factorize_naive(n: u64) -> Vec<(u64, usize)> {
    let mut r = n;
    if n < 2 {
        return vec![];
    } else {
        for p in primes(n as usize) {
            let mut temp = 0;
            while r % p == 0 {
                temp += 1;
                r /= p;
            }
            if temp > 0 {
                let mut out = vec![(p, temp)];
                out.append(&mut factorize_naive(r));
                return out;
            }
        }
    }
    vec![(n, 1)]
}
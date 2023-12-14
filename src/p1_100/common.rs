use divisors::get_divisors;
use num::Num;

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

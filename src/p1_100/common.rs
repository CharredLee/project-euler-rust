pub fn triangle(n: u64) -> u64 {    // the nth triangular number.
    n * (n + 1) / 2
}

pub fn multiple_count(n: u64, limit: u64) -> u64 {  // the number of multiples of n less than limit.
    limit / n
}

pub fn multiple_sum(n: u64, limit: u64) -> u64 {    // the sum of all multiples of n less than limit.
    n * triangle(multiple_count(n, limit))
}

pub fn square_sum(n: u64) -> u64 {  // the sum of squares through n: 1^2 + ... + n^2
    n * (n + 1) * (2*n + 1) / 6
}

pub fn euler_method(m: u64, n: u64) -> Vec<u64> {
    assert!(m > n);
    vec![m*m - n*n, 2*m*n, m*m+n*n]
}

fn get_factors(n: u64) -> Vec<u64> { // all factors  
    (1..n + 1).into_iter().filter(|&x| n % x == 0).collect::<Vec<u64>>()
}
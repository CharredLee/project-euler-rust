use std::cmp::Ordering;

use super::common::euler_method;
use divisors::get_divisors;

/*
a pythagorean triple (a, b, c) = (m^2-n^2, 2mn, m^2+n^2) satisfies
a + b + c = 1000 when 2m^2 + 2mn = 1000, when
m^2 + mn = 500.
n = (500 - m^2)/m > 0.
22^2 = 484 is the largest square below 500, so m <= 22.
must also verify that n < m, gcd(m, n) == 1, and exactly one of m, n is even.
*/

pub fn problem() {
    let mut flag = false;
    let factors = get_divisors(1000);
    let mut numerator: u64;
    let mut n: u64;
    for m in 1_u64..23 {
        let m2 = m * m;
        if flag {
            break;
        }
        for k in &factors {
            if *k < m2 {
                continue;
            }
            numerator = k - m2;
            n = numerator / m;
            if (m > n) && (numerator > m) && (numerator % m == 0) && ((m % 2 == 0) ^ (n % 2 == 0)) {
                flag = true;
                let triple = euler_method(m, n);
                let answer = triple[0] * triple[1] * triple[2];
                println!("answer: {answer}");
                break;
            }
        }
    }
}

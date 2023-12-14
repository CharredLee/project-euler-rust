use num::BigInt;

use super::common::big_fib;

pub fn problem() {
    let mut n = BigInt::from(1);
    let mut f = big_fib(&n);
    while f.to_string().len() < 1000 {
        n += 1;
        f = big_fib(&n);
    }
    let answer = n;
    println!("answer: {}", answer)
}

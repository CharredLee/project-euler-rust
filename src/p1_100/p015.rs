use num::BigUint;
use cached::proc_macro::cached;

pub fn problem() {
    let forty = BigUint::from(40u64);
    let twenty = BigUint::from(20u64);
    let answer = factorial_naive(forty) / (factorial_naive(twenty).pow(2));
    println!("answer: {answer}");
}

#[cached]
fn factorial_naive(n: BigUint) -> BigUint {
    match n == BigUint::from(1u64) {
        true => n,
        false => &n * factorial_naive(&n - BigUint::new(vec![1]))
    }
}
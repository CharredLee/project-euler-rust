use num::BigUint;
use cached::proc_macro::cached;

pub fn problem() {
    let forty = BigUint::try_from(40).unwrap();
    let twenty = BigUint::try_from(20).unwrap();
    let answer = factorial_naive(forty) / (factorial_naive(twenty).pow(2));
    println!("answer: {answer}");
}

#[cached]
fn factorial_naive(n: BigUint) -> BigUint {
    match n == BigUint::new(vec![1]) {
        true => n,
        false => &n * factorial_naive(&n - BigUint::new(vec![1]))
    }
}
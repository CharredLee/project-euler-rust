use num::BigUint;

pub fn problem() {
    let two = BigUint::from(2u64);
    let answer: u64 = two.pow(1000)
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .sum();
    println!("answer: {answer}");
}
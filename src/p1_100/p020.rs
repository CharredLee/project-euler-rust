use num::BigInt;

pub fn problem() {
    let mut n = BigInt::from(1);
    for i in 1..=100 {
        let mut temp = i;
        while temp % 10 == 0 {
            temp /= 10;
        }
        n *= temp;
    }
    let answer = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();
    println!("answer: {}", answer);
}

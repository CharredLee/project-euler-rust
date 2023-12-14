use super::common::factorial;

pub fn problem() {
    let mut target = 1_000_000 - 1;
    let mut digits = (0..10).collect::<Vec<u8>>();
    let mut current_digit = 9;
    let mut iter = 0;
    while target > 0 {
        while factorial(current_digit) <= target {
            target -= factorial(current_digit);
            digits.swap(
                9 - current_digit as usize,
                iter + (10 - current_digit) as usize,
            );
            iter += 1;
        }
        current_digit -= 1;
        iter = 0;
    }
    let answer = digits
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("answer: {}", answer);
}

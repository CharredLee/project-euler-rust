use reikna::factor;

use super::common::factorial;

pub fn problem() {
    // (0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
    // (0, 1, 2, 3, 4, 5, 6, 7, 9, 8)

    // Since 9! = 362880, there are 362880 permutations starting with 0,
    // and also 362880 permutations starting with 1, and so on.
    let mut target = 1_000_000 - 1;
    let mut digits = (0..10).collect::<Vec<u8>>();
    let mut current_digit = 9;
    let mut iter = 0;
    while target > 0 {
        while factorial(current_digit) <= target {
            target -= factorial(current_digit);
            digits.swap(9 - current_digit as usize, iter + (10 - current_digit) as usize);
            iter += 1;
        }
        current_digit -= 1;
        iter = 0;
    }
    let answer = digits.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    println!("answer: {}", answer);
}
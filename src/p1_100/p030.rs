use itertools::Itertools;

pub fn problem() {
    // 9^5 * 6 = 354294 is our upper bound, since 9^5 * 7 = 413343 is a 6-digit number

    let answer: u64 = (2..=354294)
        .filter(|&n|
            n == n
                .to_string()
                .chars()
                .map(|c| (c as u64 - 48).pow(5))
                .sum()
            )
        .sum();
    println!("answer: {}", answer);
}
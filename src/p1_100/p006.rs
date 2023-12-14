use super::common::{triangle, square_sum};

pub fn problem() {
    let answer = triangle(100u64).pow(2) - square_sum(100);
    println!("answer: {answer}");
}

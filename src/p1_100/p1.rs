use super::common::multiple_sum;

pub fn problem() {
    const LIMIT: u64 = 1000 - 1;

    let ans = multiple_sum(5, LIMIT) + multiple_sum(3, LIMIT) - multiple_sum(15, LIMIT);
    println!("answer: {ans}");
}

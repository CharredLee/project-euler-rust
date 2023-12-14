use fast_fibonacci::{self, fib_with_mod};

pub fn problem() {
    const LIMIT: u64 = 4 * 10_u64.pow(6);

    let mut answer = 0;
    let mut i = 3;
    let mut x = fib_with_mod(i, u64::MAX);
    while x < LIMIT {
        answer += x;
        i += 3;
        x = fib_with_mod(i, u64::MAX);
    }
    println!("The answer is {answer}");
}

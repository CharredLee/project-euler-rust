use super::common::collatz;
use cached::proc_macro::cached;

// this is the first problem to take more than 1 second to run, but I am uninterested in further optimization.
pub fn problem() {
    let answer = ((1u64 + 10u64.pow(6) / 2)..(10u64.pow(6)))
        .step_by(2)
        .max_by_key(|n| chain(*n))
        .unwrap();
    println!("answer: {answer}");
}

#[cached]
fn chain(n: u64) -> u64 {
    match n {
        1 => 1,
        _ => 1 + chain(collatz(n)),
    }
}

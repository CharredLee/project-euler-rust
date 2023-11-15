
pub fn triangle(n: u32) -> u32 {    // the nth triangular number.
    n * (n + 1) / 2
}

pub fn multiple_count(n: u32, limit: u32) -> u32 {  // the number of multiples of n less than limit.
    limit / n
}

pub fn multiple_sum(n: u32, limit: u32) -> u32 {    // the sum of all multiples of n less than limit.
     n * triangle(multiple_count(n, limit))
}


pub fn p() {
    const LIMIT: u32 = 1000 - 1;

    let ans = multiple_sum(5, LIMIT) + multiple_sum(3, LIMIT) - multiple_sum(15, LIMIT);
    println!("answer: {ans}") // answer: 233168
}

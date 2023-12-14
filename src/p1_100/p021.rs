// there is a weird bit shift overflow error in this problem using the divisors crate,
// so I'm implementing a naive divisor sum function to see if it fixes things.
// EDIT: it did fix things. I don't care about implementing a faster divisor search,
// at least not for this problem.
fn naive_sum_proper_divisors(n: u64) -> u64 {
    (1..n).filter(|&x| n % x == 0).sum::<u64>()
}

pub fn problem() {
    let answer = (2..10000)
        .filter(|&n| {
            let m = naive_sum_proper_divisors(n);
            n != m && n == naive_sum_proper_divisors(m)
        })
        .sum::<u64>();

    println!("answer: {}", answer);
}

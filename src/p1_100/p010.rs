use reikna::prime::{self, prime_sieve};

pub fn problem() {
    let answer: u64 = prime_sieve(2_000_000).iter().sum();
    println!("answer: {answer}");
}
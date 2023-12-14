use reikna::prime::{self, prime_sieve};

pub fn problem() {
    /*
    there are ~ N/log(N) primes less than N; for N = 10^6, log(N) is slightly less than 14,
    so there are roughly 10^6/14 ~ 71,000 primes below 10^6. So we can just check the 10,001st entry
    in the list of all primes below 10^6, and that will certainly suffice.
    */
    const LIMIT: u64 = 10_u64.pow(6);
    let primes = prime_sieve(LIMIT);
    let answer = primes[10_000]; // indexing starts at 0
    println!("answer: {answer}");
}

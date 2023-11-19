use super::common::triangle;
use divisors::get_divisors;


pub fn problem() {
    let mut n = 28;
    let mut x = triangle(n);
    while get_divisors(x).len() < 500 {
        n += 1;
        x += n;  // t(n) - t(n-1) = (n+1) * n/2 - (n-1) * n/2 = 2 * (n+1)/2 = n
    }
    let answer = x;
    println!("answer: {answer}");
}
use super::common::{square_sum, triangle};

pub fn problem() {
    /*
     * Ring i has dimensions (2i+1)x(2i+1), and the numbers in ring i are
     * those in the range (2i-1)^2 < x <= (2i+1)^2.
     * For i>0, let x_i = (2i-1)^2 (the previous square-valued corner).
     * Then the corner values in ring i are x_i+2i, x_i+4i, x_i+6i, x_i+8i.
     * Summing these values (and algebraically manipulating) gives 4(4i^2+i+1).
     * Therefore, the answer is 1 + 4 * sum_(i in 1..=500)(4i^2+4i+1)
     * = 1 + 16 * square_sum(500) + 4 * triangle(500) + 4 * 500.
     */

    let answer = 1 + 16 * square_sum(500) + 4 * triangle(500) + 4 * 500;
    println!("answer: {}", answer);
}


use reikna::factor::lcm;

pub fn problem() {
    let mut answer = 1;
    for x in 1..21 {
        answer = lcm(x, answer);
    }
    println!("answer: {answer}")
}

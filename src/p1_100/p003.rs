use reikna::factor;


pub fn problem() {
    let num: i64 = 600_851_475_143;
    let f = factor::quick_factorize(600_851_475_143);
    let answer = f[f.len() - 1];
    println!("answer: {answer}");
}
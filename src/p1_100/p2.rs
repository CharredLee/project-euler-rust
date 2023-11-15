


fn fib(n: u32) -> u32 {
    let start: [u32; 6] = [0, 1, 1, 2, 3, 5];
    let phi: f64 = (1_f64 + (5_f64).sqrt()) / 2_f64;

    if n < 6 {
        let n: usize = n as usize;
        start[n]
    } else {
        let n: f64 = n as f64;
        (phi.powf(n) / (5_f64).sqrt()) as u32
    }
}



/*
    We first want to find the largest n such that fib(n) < LIMIT.
    Since fib(n) is asymptotically phi^n/sqrt(5), we can find this very quickly.
*/
fn find_fib_max(limit: u32) -> u32 {
    if limit < 1 {
        println!("the limit should be at least 1.");
        return 0;
    }
    let approx = f64::sqrt(5.0) * (limit as f64);
    let _m = u32::max((approx - 5.0) as u32, 0);
    let _check = false;
    while !_check {
        let f = fib(_m);
        if f > limit {
            let _check = true;
        }
        let _m = _m + 1;
    };
    _m
}

pub fn problem() {
    const LIMIT: u32 = 4 * 10_u32.pow(6);

    let answer = fib(155);
    println!("F(155) = {answer}");
}
pub fn problem() {
    let mut answer = (7..1000)
        .map(|n| (n, length(n)))
        .max_by_key(|&(_, l)| l)
        .unwrap()
        .0;
    println!("answer: {}", answer);
}

pub fn length(n: u32) -> u32 {
    let mut r = 1;
    let mut d = 1usize;
    let mut remainders = vec![0; n as usize];
    while remainders[d] == 0 && d != 0 {
        remainders[d] = r;
        d = d * 10 % (n as usize);
        r += 1;
    }
    if d == 0 {
        0
    } else {
        r - remainders[d]
    }
}

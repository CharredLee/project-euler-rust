fn abundant_sum_nums(limit: usize) -> Vec<bool> {
    let mut out = vec![false; limit];
    let mut divisor_sums = vec![0; limit];
    for i in 1..limit {
        for j in (2 * i..limit).step_by(i) {
            divisor_sums[j] += i;
        }
    }
    let abundants = divisor_sums
        .iter()
        .enumerate()
        .filter(|&(i, &x)| x > i)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    for (x, y) in abundants
        .iter()
        .enumerate()
        .flat_map(|(i, &x)| abundants[i..].iter().map(move |&y| (x, y)))
    {
        if x + y < limit {
            out[x + y] = true;
        }
    }
    out
}

pub fn problem() {
    let limit = 28123 + 1;

    let answer = abundant_sum_nums(limit)
        .iter()
        .enumerate()
        .filter(|&(_, &x)| !x)
        .map(|(i, _)| i)
        .sum::<usize>();
    println!("answer: {}", answer);
}

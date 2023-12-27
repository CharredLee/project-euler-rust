use super::common::factorize_naive;
use itertools::Itertools;

pub fn problem() {
    let answer = (2..=100)
        .map(factorize_naive)
        .cartesian_product(2..=100)
        .map(|(x, b)| 
            x.into_iter().map(|(p, e)| (p, e * b)).collect_vec()
        )
        .unique()
        .count();
    println!("answer: {}", answer);
}

pub fn test() {
    for i in 2..=100 {
        println!("{i}, {:?}", factorize_naive(i));
    }
}


pub fn problem() {
    // ways[i] is the number of ways to make i pence with every coin. 200p is 2 pounds
    let mut ways = vec![1; 201];
    let coins = [2, 5, 10, 20, 50, 100, 200];
    for coin in coins {
        for i in coin..201 {
            ways[i] += ways[i - coin];
        }
    }
    let answer = ways[200];
    println!("answer: {}", answer);
}

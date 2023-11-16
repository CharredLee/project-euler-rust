// returns true <=> n is a palindrome.
fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    let s0: String = s.rsplit("").collect();
    s == s0
}


pub fn problem() {
    let mut flag = false;
    let mut answer: u32;
    for x in (900..1000).rev() {
        if flag {break}
        for y in (900..1000).rev() {
            if is_palindrome(x * y) {
                flag = true;
                answer = x * y;
                println!("answer: {answer}");
                break
            }
        }
    }
}
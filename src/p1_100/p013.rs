use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn problem() {
    let path = Path::new("problem_files/p013.txt");
    let display = path.display();

    let mut val = 0;

    if let Ok(lines) = read_lines("problem_files/p013.txt") {
        for line in lines {
            if let Ok(good_line) = line {
                val += good_line[0..12].parse::<u64>().unwrap();
            } else {
                println!("Error on line! Answer is not trusted.");
            }
        }
    }
    let answer = val.to_string()[..10].to_string();
    println!("answer: {answer}");
}

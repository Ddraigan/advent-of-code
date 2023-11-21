use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("src/day_two.txt").expect("Unable to read file");
    let lines: Vec<&str> = contents.split('\r').collect();
    println!("{:?}", lines);
    let mut trimmed_lines = vec![];
    for line in lines {
        trimmed_lines.push(line.trim())
    }
    println!("{:?}", trimmed_lines)
}

enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    fn into_rps(char: char) -> Option<RPS> {
        match char {
            'A' | 'X' => Some(RPS::Rock),
            'B' | 'Y' => Some(RPS::Paper),
            'C' | 'Z' => Some(RPS::Scissors),
            _ => None,
        }
    }
}

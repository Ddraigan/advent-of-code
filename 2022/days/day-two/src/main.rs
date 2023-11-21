use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("src/day_two.txt").expect("Unable to read file");
    let rps_strategy = convert_to_rps(contents);
    println!("{:?}", rps_strategy)
}

fn convert_to_rps(lines: String) -> Vec<(RPS, RPS)> {
    let lines: Vec<&str> = lines.split('\r').collect();
    let mut converted_lines = vec![];
    for line in lines {
        let trimmed_line = line.trim();
        let mut split = trimmed_line.split_whitespace();
        let a = RPS::try_from(split.next().unwrap().chars().nth(0).unwrap()).unwrap();
        let b = RPS::try_from(split.next().unwrap().chars().nth(0).unwrap()).unwrap();
        converted_lines.push((a, b))
    }
    converted_lines
}

#[derive(Debug)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<char> for RPS {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(RPS::Rock),
            'B' | 'Y' => Ok(RPS::Paper),
            'C' | 'Z' => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

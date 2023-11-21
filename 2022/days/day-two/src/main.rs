use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("src/day_two.txt").expect("Unable to read file");
    let rps_strategy = convert_to_rps(contents);
    let points = play_match(rps_strategy);
    println!("{}", points)
}

fn convert_to_rps(lines: String) -> Vec<(RPS, RPS)> {
    let lines: Vec<&str> = lines.split("\r\n").collect();
    let mut converted_lines = vec![];
    for line in lines {
        let mut split = line.split_whitespace();
        let a = RPS::try_from(split.next().unwrap().chars().nth(0).unwrap()).unwrap();
        let b = RPS::try_from(split.next().unwrap().chars().nth(0).unwrap()).unwrap();
        converted_lines.push((a, b))
    }
    converted_lines
}

fn play_match(rps_strategy: Vec<(RPS, RPS)>) -> usize {
    let mut points: usize = 0;

    for game in rps_strategy {
        points += get_winner(&game.0, &game.1);
        let x = game.1 as usize;
        // println!("{x}");
        points += x
    }

    points
}

fn get_winner(opponent: &RPS, player: &RPS) -> usize {
    let x = if opponent < player {
        if player == &RPS::Rock {
            0
        } else {
            6
        }
    } else if opponent > player {
        if player == &RPS::Scissors {
            6
        } else {
            0
        }
    } else {
        3
    };
    // println!("{:#?}", x);
    x
}

#[derive(Debug, PartialEq, PartialOrd)]
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

// "6 3 3 6 0 3 3 6 0 3 3 0"

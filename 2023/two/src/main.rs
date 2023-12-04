use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    vec,
};

fn main() {
    let lines = lines_from_file("src/input.txt").expect("Path to have a readable file");

    let record = Record::new(lines);

    println!("{:?}", record.possible_games());
}

#[derive(Debug)]
struct Record {
    games: Vec<CubeGame>,
}

impl Record {
    fn new(lines: Vec<String>) -> Self {
        let mut games = vec![];

        for line in lines {
            games.push(CubeGame::new(&line))
        }

        Self { games }
    }

    fn possible_games(&self) -> usize {
        let mut possible_game_ids = vec![];

        for game in &self.games {
            if !game.is_max() {
                // println!("{:?}", game.game_id);
                possible_game_ids.push(game.game_id as usize)
            }
        }

        possible_game_ids.iter().sum()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct CubeGame {
    game_id: u8,
    rounds: Vec<Round>,
}

impl CubeGame {
    fn new(line: &str) -> Self {
        let game_id = Self::parse_game_id(line);
        let rounds = Self::parse_rounds(line);

        Self { game_id, rounds }
    }

    fn is_max(&self) -> bool {
        let mut is_max = false;

        for round in &self.rounds {
            if round.is_max() {
                is_max = true;
                break;
            }
        }

        is_max
    }

    fn parse_game_id(line: &str) -> u8 {
        line.chars()
            .into_iter()
            .take_while(|c| *c != ':')
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .expect("Parsable number")
    }

    fn parse_rounds(line: &str) -> Vec<Round> {
        line.split(|c| c == ':' || c == ';')
            .skip(1)
            .map(|l| l.trim())
            .map(|item| Round::new(item))
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Round {
    cubes: Vec<Cube>,
}

impl Round {
    fn new(round: &str) -> Self {
        let cubes = Self::parse_cubes(round);
        Self { cubes }
    }

    fn is_max(&self) -> bool {
        let mut is_max = false;

        for cube in &self.cubes {
            if cube.is_max() {
                is_max = true;
                break;
            }
        }

        is_max
    }

    fn parse_cubes(round: &str) -> Vec<Cube> {
        round
            .split(',')
            .map(|e| e.trim())
            .map(|cube| Cube::new(cube))
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Cube {
    amount: u8,
    colour: Colour,
}

impl Cube {
    fn new(data: &str) -> Self {
        let amount = Self::parse_amount(data);
        let colour = Self::parse_colour(data);

        Self { amount, colour }
    }

    fn is_max(&self) -> bool {
        if self.amount > self.colour.max() {
            return true;
        } else {
            return false;
        }
    }

    fn parse_amount(data: &str) -> u8 {
        let data: Vec<&str> = data.split(' ').collect();
        data[0].parse().expect("Parsable number")
    }

    fn parse_colour(data: &str) -> Colour {
        let data: Vec<&str> = data.split(' ').collect();
        data[1].try_into().expect("Valid colour (red, blue, green)")
    }
}

#[derive(Debug)]
enum Colour {
    Red,
    Blue,
    Green,
}

impl Colour {
    fn max(&self) -> u8 {
        match *self {
            Colour::Red => 12,
            Colour::Blue => 14,
            Colour::Green => 13,
        }
    }
}

impl TryFrom<&str> for Colour {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Colour::Red),
            "blue" => Ok(Colour::Blue),
            "green" => Ok(Colour::Green),
            _ => Err(()),
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    buf.lines().collect()
}

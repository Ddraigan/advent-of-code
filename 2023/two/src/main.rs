use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("src/input.txt").expect("Path to have a readable file");
    let record = Record::new(lines);

    println!("{:?}", record.possible_games());
    println!("{}", record.sum_round_powers())
}

#[derive(Debug)]
struct Record {
    games: Vec<CubeGame>,
}

impl Record {
    fn new(lines: Vec<String>) -> Self {
        let games = lines.iter().map(|line| CubeGame::new(&line)).collect();

        Self { games }
    }

    fn sum_round_powers(&self) -> usize {
        self.power_of_rounds().iter().sum()
    }

    fn power_of_rounds(&self) -> Vec<usize> {
        self.games
            .iter()
            .map(|game| {
                CubeGame::largest_cubes(game.join_rounds())
                    .iter()
                    .map(|num| *num as usize)
                    .product()
            })
            .collect()
    }

    fn possible_games(&self) -> u32 {
        self.games
            .iter()
            .filter(|game| !game.is_max())
            .fold(0, |acc, x| acc + x.game_id as u32)
    }
}

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

    fn largest_cubes(cubes: Vec<&Cube>) -> Vec<u8> {
        let mut largest = vec![];

        for colour in Colour::COLOURS {
            largest.push(Self::largest_cube(&cubes, colour).expect("Tester"))
        }

        largest
    }

    fn largest_cube(cubes: &Vec<&Cube>, colour: Colour) -> Option<u8> {
        cubes
            .iter()
            .filter(|cube| cube.colour == colour)
            .map(|cube| cube.amount)
            .into_iter()
            .max()
    }

    fn join_rounds(&self) -> Vec<&Cube> {
        self.rounds
            .iter()
            .map(|round| &round.cubes)
            .into_iter()
            .flatten()
            .collect()
    }

    fn is_max(&self) -> bool {
        self.rounds.iter().any(|round| round.is_max())
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
            .map(|item| Round::new(item.trim()))
            .collect()
    }
}

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
        self.cubes.iter().any(|cube| cube.is_max())
    }

    fn parse_cubes(round: &str) -> Vec<Cube> {
        round
            .split(',')
            .map(|cube| Cube::new(cube.trim()))
            .collect()
    }
}

#[derive(Debug)]
struct Cube {
    amount: u8,
    colour: Colour,
}

impl Cube {
    fn new(cube_info: &str) -> Self {
        let amount = Self::parse_amount(cube_info);
        let colour = Self::parse_colour(cube_info);

        Self { amount, colour }
    }

    fn is_max(&self) -> bool {
        if self.amount > self.colour.max() {
            return true;
        } else {
            return false;
        }
    }

    fn parse_amount(cube_info: &str) -> u8 {
        let data: Vec<&str> = cube_info.split(' ').collect();
        data[0].parse().expect("Parsable number")
    }

    fn parse_colour(cube_info: &str) -> Colour {
        let data: Vec<&str> = cube_info.split(' ').collect();
        data[1].try_into().expect("Valid colour (red, blue, green)")
    }
}

#[derive(Debug, PartialEq)]
enum Colour {
    Red,
    Green,
    Blue,
}

impl Colour {
    const COLOURS: [Colour; 3] = [Colour::Red, Colour::Green, Colour::Blue];

    fn max(&self) -> u8 {
        match *self {
            Colour::Red => 12,
            Colour::Green => 13,
            Colour::Blue => 14,
        }
    }
}

impl TryFrom<&str> for Colour {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Colour::Red),
            "green" => Ok(Colour::Green),
            "blue" => Ok(Colour::Blue),
            _ => Err(()),
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    buf.lines().collect()
}

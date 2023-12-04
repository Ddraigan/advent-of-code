use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("src/input.txt").expect("Path to have a readable file");
    println!("{:?}", lines);

    let record = Record::new(lines);

    println!("{}", record.possible_games())
}

#[derive(Debug)]
struct Record(Vec<CubeGame>);

impl Record {
    fn new(lines: Vec<String>) -> Self {
        let mut record = vec![];

        for line in lines {
            record.push(CubeGame::new(&line))
        }
        Record(record)
    }

    fn possible_games(&self) -> usize {
        let mut game_ids = vec![];

        for game in &self.0 {
            game.pulls.iter().for_each(|pull| {
                pull.cubes.iter().for_each(|cube| {
                    if cube.amount <= cube.colour.max() {
                        game_ids.push(game.game_id as usize)
                    }
                })
            });
        }

        game_ids.iter().sum()
    }
}

#[derive(Debug)]
struct CubeGame {
    game_id: u8,
    pulls: Vec<Pull>,
}

impl CubeGame {
    fn new(line: &str) -> Self {
        let game_id = Self::parse_game_id(line);
        let pulls = Self::parse_pulls(line);

        Self { game_id, pulls }
    }

    fn parse_game_id(line: &str) -> u8 {
        let id: String = line
            .chars()
            .into_iter()
            .take_while(|c| *c != ':')
            .filter(|c| c.is_ascii_digit())
            .collect();

        id.parse().expect("Parsable number")
    }

    fn parse_pulls(line: &str) -> Vec<Pull> {
        let pull = line
            .split(|c| c == ':' || c == ';')
            .skip(1)
            .map(|l| l.trim())
            .collect::<Vec<&str>>();

        let mut pulls = vec![];

        for p in pull {
            pulls.push(Pull::new(p))
        }

        pulls
    }
}

#[derive(Debug)]
struct Pull {
    cubes: Vec<Cube>,
}

impl Pull {
    fn new(pull: &str) -> Self {
        let cubes = Self::parse_cubes(pull);
        Self { cubes }
    }

    fn parse_cubes(pull: &str) -> Vec<Cube> {
        let cube = pull.split(',').map(|e| e.trim()).collect::<Vec<&str>>();

        let mut cubes = vec![];

        for c in cube {
            cubes.push(Cube::new(c))
        }

        cubes
    }
}

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

    fn parse_amount(data: &str) -> u8 {
        let amount = data
            .chars()
            .find(|c| c.is_ascii_digit())
            .expect("Value to be present");

        amount.to_string().parse().expect("Parsable number")
    }

    fn parse_colour(data: &str) -> Colour {
        let data: Vec<&str> = data.split(' ').collect();
        let colour = data[1].try_into().expect("Valid colour (red, blue, green)");
        colour
    }
}

#[derive(Debug)]
enum Colour {
    Red,
    Blue,
    Green,
}

impl Colour {
    const COLOURS: [&str; 3] = ["red", "blue", "green"];

    fn value(&self) -> &str {
        match self {
            Colour::Red => "red",
            Colour::Blue => "blue",
            Colour::Green => "green",
        }
    }

    fn max(&self) -> u8 {
        match self {
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

#[test]
fn parse_pulls_test() {
    let lines = lines_from_file("src/input.txt").unwrap();
    for line in lines {
        let pull = line
            .split(|c| c == ':' || c == ';')
            .skip(1)
            .map(|l| l.trim())
            .collect::<Vec<&str>>();
        println!("{:?}", pull)
    }
}

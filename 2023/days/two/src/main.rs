use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("src/input.txt").expect("Path to have a readable file");
    println!("{:?}", lines);

    // let record = Record::new(&lines);
    // println!("{:?}", record)
}

struct Record(Vec<CubeGame>);

struct CubeGame {
    game_id: u8,
    pulls: Vec<Pull>,
}

impl CubeGame {
    fn new(line: &str) -> Self {
        let game_id = CubeGame::parse_game_id(line);
        let pulls = CubeGame::parse_pulls(line);

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

        for item in pull {
            pulls.push(Pull::new(item))
        }

        pulls
    }
}

struct Pull {
    cubes: Vec<Cube>,
}

impl Pull {
    fn new(line: &str) -> Self {
        let cubes = parse_cubes(line);
        Self { cubes }
    }
}

struct Cube {
    amount: u8,
    colour: Colour,
}

impl Cube {
    fn new(amount: u8, colour: &str) -> Self {
        let colour = colour.try_into().expect("Valid colour option");

        Self { amount, colour }
    }
}

#[derive(Debug)]
enum Colour {
    Red,
    Blue,
    Green,
}

impl Colour {
    fn value(&self) -> &str {
        match self {
            Colour::Red => "red",
            Colour::Blue => "blue",
            Colour::Green => "green",
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

// #[test]
// fn parse_game_id_test() {
//     let lines = lines_from_file("src/input.txt").unwrap();
//     for line in lines {
//         let id = parse_game_id(&line);
//         println!("{}", id)
//     }
// }

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

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("src/input.txt").expect("Path to have a readable file");

    let record = Record::new(lines);

    println!("{:#?}", record)
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
}

#[allow(dead_code)]
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
        line.chars()
            .into_iter()
            .take_while(|c| *c != ':')
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .expect("Parsable number")
    }

    fn parse_pulls(line: &str) -> Vec<Pull> {
        line.split(|c| c == ':' || c == ';')
            .skip(1)
            .map(|l| l.trim())
            .map(|item| Pull::new(item))
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Pull {
    cubes: Vec<Cube>,
}

impl Pull {
    fn new(pull: &str) -> Self {
        let cubes = Self::parse_cubes(pull);
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

    fn parse_cubes(pull: &str) -> Vec<Cube> {
        pull.split(',')
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
        data.chars()
            .find(|c| c.is_ascii_digit())
            .expect("Value to be present")
            .to_string()
            .parse()
            .expect("Parsable number")
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

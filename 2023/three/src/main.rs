use std::{fs, path::Path};

fn main() {
    let lines = file_to_string("src/input.txt");
    let grid = Grid::new(&lines);

    println!("{:#?}", grid.symbols());
}

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
    value: char,
}

impl Point {
    fn new(x: u8, y: u8, value: char) -> Self {
        Self { x, y, value }
    }
}

#[derive(Debug)]
struct Grid {
    content: Vec<Vec<char>>,
}

impl Grid {
    fn new(content: &str) -> Self {
        let content = Self::parse_content(content);

        Self { content }
    }

    fn symbols(&self) -> Vec<Point> {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_x, char)| {
                        (**char as u8) == 61
                            || (**char as u8) == 64
                            || (**char as u8) < 48 && (**char as u8) != 46
                    })
                    .map(|(x, char)| {
                        Point::new(x.try_into().unwrap(), y.try_into().unwrap(), *char)
                    })
                    .collect::<Vec<Point>>()
            })
            .collect()
    }

    fn parse_content(content: &str) -> Vec<Vec<char>> {
        content
            .lines()
            .into_iter()
            .map(|line| line.chars().collect())
            .collect()
    }
}

fn file_to_string(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path).expect("File to be there")
}

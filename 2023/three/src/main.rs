use std::{fs, path::Path};

fn main() {
    let lines = file_to_string("src/input.txt");
    let grid = Grid::new(&lines);

    println!("{:#?}", grid.find_symbols());
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

    fn part_one(&self) {
        let neighbours: [(i32, i32); 8] = [
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];
    }

    fn find_symbols(&self) -> Vec<Point> {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, char)| {
                    if (*char as u8) == 61
                        || (*char as u8) == 64
                        || (*char as u8) < 48 && (*char as u8) != 46
                    {
                        Some(Point::new(
                            x.try_into().expect("Number to be within bounds of u8"),
                            y.try_into().expect("Number to be within bounds of u8"),
                            *char,
                        ))
                    } else {
                        None
                    }
                })
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
    fs::read_to_string(path).expect("File to be exist")
}

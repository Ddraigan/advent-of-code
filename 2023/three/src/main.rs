use std::{fs, path::Path};

fn main() {
    let lines = file_to_string("src/input.txt");
    let grid = Grid::new(&lines);
    println!("{:?}", grid);
    println!("{:?}", grid.content[0].len())
}

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
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

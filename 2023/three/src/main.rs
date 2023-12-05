use std::{fs, path::Path};

fn main() {
    println!("{:?}", lines_from_file("src/input.txt"))
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
    // fn new(content: String) -> Self {
    //     let content = Self::parse_content(content);
    //
    //     Self { content }
    // }

    fn parse_content(content: String) {}
}

fn lines_from_file(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path).expect("File to be there")
}

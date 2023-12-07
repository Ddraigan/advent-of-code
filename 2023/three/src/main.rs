use std::{fs, path::Path};

fn main() {
    let lines = file_to_string("src/input.txt");
    let grid = Grid::new(&lines);

    println!("{:#?}", grid.points_at_symbol());
}

// for current_point in point.neighbours() {
//     if !self.val_at_point(&current_point).is_ascii_digit() {
//         continue;
//     }

#[derive(Debug)]
struct Grid {
    content: Vec<Vec<char>>,
}

impl Grid {
    fn new(content: &str) -> Self {
        let content = Self::parse_content(content);

        Self { content }
    }

    fn points_at_symbol(&self) -> usize {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_x, char)| char.is_symbol())
                    .map(move |(x, _char)| {
                        Point::new(
                            x.try_into().expect("Within bounds of u8"),
                            y.try_into().expect("Within bounds of u8"),
                        )
                        .neighbours()
                        .iter()
                        .filter(|current_point| self.val_at_point(current_point).is_ascii_digit())
                    })
            })
            .sum()
    }

    fn take_replace_val_at_point(&mut self, point: &Point) -> char {
        let og_val = self.content[point.y as usize][point.x as usize];

        self.content[point.y as usize][point.x as usize] = '.';

        og_val
    }

    fn val_at_point(&self, point: &Point) -> &char {
        &self.content[point.y as usize][point.x as usize]
    }

    fn parse_content(content: &str) -> Vec<Vec<char>> {
        content
            .lines()
            .into_iter()
            .map(|line| line.chars().collect())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn next_x(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn next_y(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn prev_x(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn prev_y(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn neighbours(&self) -> [Point; 8] {
        let neighbours: [Point; 8] = [
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
        ];
        neighbours
    }
}

trait Symbol {
    fn is_symbol(&self) -> bool;
}

impl Symbol for char {
    fn is_symbol(&self) -> bool {
        if (*self as u8) == 61 || (*self as u8) == 64 || (*self as u8) < 48 && (*self as u8) != 46 {
            return true;
        } else {
            return false;
        }
    }
}

fn file_to_string(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path).expect("File to be exist")
}

// #[test]
// fn part_one_test() {
//     let control = [
//         Point { x: 121, y: 139 },
//         Point { x: 121, y: 138 },
//         Point { x: 121, y: 137 },
//         Point { x: 120, y: 139 },
//         Point { x: 120, y: 137 },
//         Point { x: 119, y: 137 },
//         Point { x: 119, y: 138 },
//         Point { x: 119, y: 139 },
//     ];
//
//     let lines = file_to_string("src/input.txt");
//     let grid = Grid::new(&lines);
//
//     println!("{:#?}", grid.part_one().last());
//
//     assert!(&control == grid.part_one().last().unwrap())
// }

use std::{fs, path::Path, vec};

fn main() {
    let lines = file_to_string("src/input.txt");
    let grid = Grid::new(&lines);

    // println!("{:#?}", grid.find_symbols());
    println!("{:#?}", grid.part_one());
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

#[derive(Debug)]
struct Grid {
    content: Vec<Vec<char>>,
}

impl Grid {
    fn new(content: &str) -> Self {
        let content = Self::parse_content(content);

        Self { content }
    }

    fn part_one(&self) -> usize {
        let mut numbers: Vec<usize> = vec![];

        for (y, line) in self.content.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if !char.is_symbol() {
                    continue;
                }

                let point = Point::new(
                    x.try_into().expect("Within bounds of u8"),
                    y.try_into().expect("Within bounds of u8"),
                );

                for neighbour in point.neighbours() {
                    if !self.point(&neighbour).is_ascii_digit() {
                        continue;
                    }
                    println!("{}", self.point(&neighbour))
                }
            }
        }

        numbers.into_iter().sum()
    }

    fn point(&self, point: &Point) -> &char {
        &self.content[point.y as usize][point.x as usize]
    }

    // fn find_symbols(&self) -> Vec<Point> {
    //     self.content
    //         .iter()
    //         .enumerate()
    //         .flat_map(|(y, line)| {
    //             line.iter().enumerate().filter_map(move |(x, char)| {
    //                 if (*char as u8) == 61
    //                     || (*char as u8) == 64
    //                     || (*char as u8) < 48 && (*char as u8) != 46
    //                 {
    //                     Some(Point::new(
    //                         x.try_into().expect("Number to be within bounds of u8"),
    //                         y.try_into().expect("Number to be within bounds of u8"),
    //                     ))
    //                 } else {
    //                     None
    //                 }
    //             })
    //         })
    //         .collect()
    // }

    fn parse_content(content: &str) -> Vec<Vec<char>> {
        content
            .lines()
            .into_iter()
            .map(|line| line.chars().collect())
            .collect()
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

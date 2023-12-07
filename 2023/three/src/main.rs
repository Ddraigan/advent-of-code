use std::{fs, path::Path};

fn main() {
    let lines = file_to_string("src/input.txt");
    let mut grid = Grid::new(&lines);
    let part_one = grid.sum_numbers_around_symbols();
    println!("{part_one}");
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

    fn sum_numbers_around_symbols(&mut self) -> usize {
        self.number_neigbour_points()
            .iter()
            .filter_map(|&neighbour| {
                if !self.get(&neighbour).is_ascii_digit() {
                    return None;
                }

                let (num_string_left, num_string_right) = self.numbers_around_point(&neighbour);

                let num_string = format!(
                    "{}{}{}",
                    num_string_left.chars().rev().collect::<String>(),
                    self.get(&neighbour),
                    num_string_right
                );

                num_string.parse::<usize>().ok()
            })
            .sum()
    }

    fn numbers_around_point(&mut self, neighbour: &Point) -> (String, String) {
        let mut num_string_left = String::new();
        let mut num_string_right = String::new();

        for (direction, num_string) in &mut [(-1, &mut num_string_left), (1, &mut num_string_right)]
        {
            for n in 1..=2 {
                let next_point = neighbour.add_offset(*direction * n, 0);
                if !self.get(&next_point).is_ascii_digit() {
                    break;
                }
                num_string.push(self.take(&next_point));
            }
        }

        (num_string_left, num_string_right)
    }

    fn number_neigbour_points(&self) -> Vec<Point> {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &cell)| cell.is_symbol())
                    .flat_map(move |(x, _)| {
                        Point::new(x.try_into().unwrap(), y.try_into().unwrap()).neighbours()
                    })
            })
            .collect()
    }

    fn replace(&mut self, point: &Point, char: char) {
        self.content[point.y as usize][point.x as usize] = char
    }

    fn take(&mut self, point: &Point) -> char {
        let val = self.content[point.y as usize][point.x as usize];
        self.replace(point, '.');
        val
    }

    fn get(&self, point: &Point) -> &char {
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

    fn add_offset(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn neighbours(&self) -> [Point; 8] {
        [
            self.add_offset(1, 1),
            self.add_offset(1, 0),
            self.add_offset(1, -1),
            self.add_offset(0, 1),
            self.add_offset(0, -1),
            self.add_offset(-1, -1),
            self.add_offset(-1, 0),
            self.add_offset(-1, 1),
        ]
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

#[test]
fn part_one_test() {
    let lines = file_to_string("src/input.txt");
    let mut grid = Grid::new(&lines);
    let part_one = grid.sum_numbers_around_symbols();

    assert_eq!(part_one, 551094);
}

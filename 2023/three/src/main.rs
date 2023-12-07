use std::{fs, path::Path};

fn main() {
    let lines = file_to_string("src/input.txt");
    let mut grid = Grid::new(&lines);
    let part_one = points_at_symbol(&mut grid);
    println!("{part_one}");
}

fn points_at_symbol(grid: &mut Grid) -> usize {
    let content = &mut grid.content;
    let height = content.len();
    let width = content[0].len();

    let mut nums: Vec<usize> = vec![];

    let mut neighbours = vec![];

    for y in 0..height {
        for x in 0..width {
            if content[y][x].is_symbol() {
                let current_point = Point::new(x.try_into().unwrap(), y.try_into().unwrap());

                neighbours.extend(
                    current_point
                        .neighbours()
                        .iter()
                        .filter(|&neighbour| {
                            let nx = neighbour.x as usize;
                            let ny = neighbour.y as usize;
                            nx < width && ny < height && content[ny][nx].is_ascii_digit()
                        })
                        .cloned()
                        .into_iter()
                        .map(move |neighbour| neighbour),
                );
            }
        }
    }

    for neighbour in neighbours {
        let neighbour_val = grid.val_at_point(&neighbour);

        if !neighbour_val.is_ascii_digit() {
            continue;
        }

        let mut num_string_left = String::from("");
        let num_string_mid = String::from(neighbour_val);
        let mut num_string_right = String::from("");

        for n in 1..3 {
            let next_point = Point::new(&neighbour.x + n, neighbour.y);
            if !grid.val_at_point(&next_point).is_ascii_digit() {
                break;
            }
            num_string_right.push(grid.val_at_point(&next_point));
            grid.replace_at_point(&next_point);
        }

        for n in 1..3 {
            let last_point = Point::new(&neighbour.x - n, neighbour.y);
            if !grid.val_at_point(&last_point).is_ascii_digit() {
                break;
            }
            num_string_left.push(grid.val_at_point(&last_point));
            grid.replace_at_point(&last_point);
        }

        let num_string = format!(
            "{}{}{}",
            num_string_left.chars().rev().collect::<String>(),
            num_string_mid,
            num_string_right
        );

        nums.push(num_string.parse::<usize>().unwrap())
    }

    nums.into_iter().sum()
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

    fn replace_at_point(&mut self, point: &Point) {
        self.content[point.y as usize][point.x as usize] = '.';
    }

    fn val_at_point(&self, point: &Point) -> char {
        self.content[point.y as usize][point.x as usize]
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

#[test]
fn part_one_test() {
    let lines = file_to_string("src/input.txt");
    let mut grid = Grid::new(&lines);
    let part_one = points_at_symbol(&mut grid);

    assert_eq!(part_one, 551094);
}

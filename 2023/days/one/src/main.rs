use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

const NUMBERWORDS: [(&'static str, usize); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    // Part One
    let lines = lines_from_file("src/input.txt").expect("File to exist");
    let numbers = part_one_parse_numbers(lines);
    let number = get_number(numbers);

    println!("{:?}", number);

    // Part Two
    let lines = lines_from_file("src/input.txt").expect("File to exist");
    let numbers = part_two_parse_numbers(&lines);
    let number = get_number(numbers);

    println!("{:?}", number);
}

fn get_number(numbers: Vec<usize>) -> usize {
    numbers.iter().sum()
}

fn part_two_parse_numbers(lines: &Vec<String>) -> Vec<usize> {
    let mut numbers = vec![];

    for line in lines {
        let mut nums_in_line: Vec<(usize, usize)> = vec![];
        let i = line.find(|c: char| c.is_ascii_digit()).unwrap();
        let v = &line.chars().nth(i).unwrap();
        nums_in_line.push((i, *v as usize));

        for (word, number) in NUMBERWORDS {
            match line.find(word) {
                Some(i) => nums_in_line.push((i, number)),
                None => {}
            }
        }

        let i = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
        let v = &line.chars().nth(i).unwrap();
        nums_in_line.push((i, *v as usize));

        nums_in_line.sort_by_key(|k| k.0);

        let first_num = nums_in_line[0].0.to_string();
        let last_num = nums_in_line.last().unwrap().0.to_string();

        let number = format!("{}{}", first_num, last_num)
            .parse()
            .expect("Valid Number");

        numbers.push(number);
    }

    numbers
}

fn part_one_parse_numbers(lines: Vec<String>) -> Vec<usize> {
    let mut numbers = vec![];

    for line in lines {
        let mut nums_in_line = vec![];
        line.chars().for_each(|c| {
            if c.is_ascii_digit() {
                nums_in_line.push(c)
            }
        });

        let first_num = nums_in_line[0];
        let last_num = nums_in_line.last().unwrap_or(&'\0').to_owned();

        let number: usize = format!("{}{}", first_num, last_num)
            .parse()
            .expect("Valid Number");

        numbers.push(number);
    }
    numbers
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    buf.lines().collect()
}

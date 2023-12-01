use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    let lines = lines_from_file("src/input.txt").expect("File to exist");
    let numbers = parse_numbers(lines);
    let number = get_number(numbers);

    println!("{:?}", number);
}

fn get_number(numbers: Vec<usize>) -> usize {
    numbers.iter().sum()
}

fn parse_numbers(lines: Vec<String>) -> Vec<usize> {
    let mut numbers = vec![];

    for line in lines {
        let mut nums_in_line = vec![];
        line.chars().for_each(|c| {
            if c.is_ascii_digit() {
                nums_in_line.push(c)
            }
        });

        let last_num = match nums_in_line.last() {
            Some(num) => num.to_string(),
            None => String::from(""),
        };

        let number: usize = String::from(nums_in_line[0].to_string() + &last_num)
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

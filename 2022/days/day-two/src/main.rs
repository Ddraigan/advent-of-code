use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("src/day_two.txt").expect("Unable to read file");
    let lines: Vec<&str> = contents.split('\r').collect();
    println!("{:?}", lines);
    let mut trimmed_lines = vec![];
    for line in lines {
        trimmed_lines.push(line.trim())
    }
    println!("{:?}", trimmed_lines)
}

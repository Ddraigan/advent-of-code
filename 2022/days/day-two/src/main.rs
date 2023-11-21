use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("src/day_one").expect("Unable to read file");
    let lines: Vec<&str> = contents.split('\n').collect();
    println!("{:?}", lines)
}

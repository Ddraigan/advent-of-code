fn main() {
    let lines = lines_from_file("src/input.txt").unwrap();

    let cards: Vec<Card> = lines.iter().map(|line| Card::from(line.as_str())).collect();

    println!("{cards:?}")
}

#[derive(Debug)]
struct Card {
    id: u8,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl Card {}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let value = value.split(&[':', '|']).collect::<Vec<_>>();

        let id = value[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let winning_numbers = value[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|item| item.parse().unwrap())
            .collect();

        let card_numbers = value[2]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|item| item.parse().unwrap())
            .collect();

        Self {
            id,
            winning_numbers,
            card_numbers,
        }
    }
}

fn lines_from_file(path: &str) -> std::io::Result<Vec<String>> {
    let file = std::fs::File::open(path)?;
    let buf = std::io::BufReader::new(file);
    std::io::BufRead::lines(buf).collect()
}

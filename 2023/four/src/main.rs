fn main() {
    let lines = lines_from_file("src/input.txt").unwrap();

    let cards: Vec<Card> = lines
        .iter()
        .map(|line| Card::try_from(line.as_str()).unwrap())
        .collect();

    println!("{cards:?}")
}

#[derive(Debug)]
struct Card {
    id: u8,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl Card {}

impl TryFrom<&str> for Card {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        let mut parts = value.splitn(3, |c| c == ':' || c == '|');

        let id = parts
            .nth(0)
            .and_then(|part| part.split_whitespace().nth(1))
            .ok_or(())?
            .parse()
            .map_err(|_| ())?;

        let parse_numbers = |s: &str| -> Result<Vec<u32>, ()> {
            s.split_whitespace()
                .map(|item| item.parse().map_err(|_| ()))
                .collect()
        };

        let winning_numbers = parse_numbers(parts.next().ok_or(())?)?;
        let card_numbers = parse_numbers(parts.next().ok_or(())?)?;

        Ok(Self {
            id,
            winning_numbers,
            card_numbers,
        })
    }
}

fn lines_from_file(path: &str) -> std::io::Result<Vec<String>> {
    let file = std::fs::File::open(path)?;
    let buf = std::io::BufReader::new(file);
    std::io::BufRead::lines(buf).collect()
}

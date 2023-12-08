fn main() {
    let lines = lines_from_file("src/input.txt").unwrap();

    let cards: Vec<Card> = lines
        .iter()
        .map(|line| line.as_str().try_into().unwrap())
        .collect();

    let mut total_points = vec![];

    for card in cards.iter() {
        total_points.push(card.matches());
        println!("{:?}", card.matches());
    }

    println!("{:?}", total_points.iter().sum::<usize>())
}

#[derive(Debug)]
struct Card {
    id: u8,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl Card {
    fn matches_iter_style(&self) -> usize {
        self.card_numbers
            .iter()
            .filter(|card_number| self.winning_numbers.contains(card_number))
            .fold(0, |match_count, _| {
                if match_count != 0 {
                    match_count * 2
                } else {
                    match_count + 1
                }
            })
    }

    fn matches(&self) -> usize {
        let mut match_count = 0;

        for card_number in &self.card_numbers {
            if self
                .winning_numbers
                .iter()
                .any(|winning_number| card_number == winning_number)
            {
                match_count = if match_count != 0 {
                    match_count * 2
                } else {
                    match_count + 1
                };
            }
        }

        match_count
    }
}

// self.card_numbers.iter().filter(|card_number| {
//     self.winning_numbers
//         .iter()
//         .any(|winning_number| *card_number == winning_number)
// })

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

#[test]
fn part_one_test() {
    let lines = lines_from_file("src/input.txt").unwrap();

    let cards: Vec<Card> = lines
        .iter()
        .map(|line| line.as_str().try_into().unwrap())
        .collect();

    let total_points: usize = cards.iter().map(|card| card.matches()).sum();
    let total_points_iter_style: usize = cards.iter().map(|card| card.matches_iter_style()).sum();

    assert_eq!(21105, total_points);
    assert_eq!(21105, total_points_iter_style);
}

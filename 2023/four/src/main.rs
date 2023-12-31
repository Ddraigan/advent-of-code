use std::ops::{Deref, DerefMut};

fn main() {
    let lines = lines_from_file("src/input.txt").unwrap();

    let cards: Deck = lines
        .iter()
        .map(|line| Card::try_from(line.as_str()).unwrap())
        .collect();

    let part_one: usize = cards.iter().map(|card| card.match_count_doubled()).sum();
    println!("{:?}", part_one);

    let part_two = cards.part_two();
    println!("{:?}", part_two);
}

struct Deck(Vec<Card>);

impl Deck {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, elem: Card) {
        self.0.push(elem);
    }

    fn part_two(&self) -> usize {
        let mut counts = vec![1; self.len()];

        for (index, card) in self.iter().enumerate() {
            let x = if card.matches() > self.len() - 1 {
                self.len() - 1
            } else {
                card.matches() + index
            };

            for y in index + 1..x + 1 {
                counts[y] += counts[index];
            }
        }

        counts.iter().sum()
    }
}

struct Card {
    id: u8,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl Card {
    fn matches(&self) -> usize {
        self.card_numbers
            .iter()
            .filter(|card_number| self.winning_numbers.contains(card_number))
            .count()
    }

    fn match_count_doubled(&self) -> usize {
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

    fn match_count_doubled_iter_style(&self) -> usize {
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
}

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

impl FromIterator<Card> for Deck {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut deck = Deck::new();

        for item in iter {
            deck.add(item)
        }

        deck
    }
}

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = <Vec<Card> as IntoIterator>::IntoIter; // so that you don't have to write std::vec::IntoIter, which nobody remembers anyway

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for Deck {
    type Target = [Card];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

    let total_points: usize = cards.iter().map(|card| card.match_count_doubled()).sum();
    let total_points_iter_style: usize = cards
        .iter()
        .map(|card| card.match_count_doubled_iter_style())
        .sum();

    assert_eq!(21105, total_points);
    assert_eq!(21105, total_points_iter_style);
}

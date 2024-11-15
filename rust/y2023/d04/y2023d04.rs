use std::collections::HashSet;
use std::str::FromStr;

pub fn part1(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let cards = parse_input(input);
    let total_score: u32 = cards.iter().map(score).sum();
    Ok(total_score.to_string())
}

pub fn part2(input: String) -> Result<String, Box<dyn std::error::Error>> {
    let cards = parse_input(input);
    let n_cards = cards.len();
    let mut counts = vec![1; n_cards];

    for (k, card) in cards.iter().enumerate() {
        for i in 0..card.n_winning() {
            let idx = k + i + 1;
            if idx < n_cards {
                counts[idx] += counts[k]
            }
        }
    }

    Ok(counts.iter().sum::<usize>().to_string())
}

#[derive(Debug)]
struct Card {
    winning: HashSet<u8>,
    numbers: Vec<u8>,
}

impl Card {
    fn n_winning(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

impl FromStr for Card {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" | ").collect();
        if parts.len() != 2 {
            return Err("Invalid card format".into());
        }
        let winning = parts[0]
            .split_whitespace()
            .skip(2) // card id
            .map(str::parse)
            .collect::<Result<HashSet<u8>, _>>()?;
        let numbers = parts[1]
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<u8>, _>>()?;

        Ok(Card { winning, numbers })
    }
}

fn parse_input(input: String) -> Vec<Card> {
    input
        .lines()
        .map(Card::from_str)
        .map(Result::unwrap)
        .collect()
}

fn score(card: &Card) -> u32 {
    (2_u32).pow(card.n_winning() as u32 - 1)
}

use std::collections::HashSet;
use std::io;
use regex::Regex;

fn get_number_array(numbers: &str) -> Vec<u32> {
    numbers.trim().split_whitespace()
        .filter(|str| !str.is_empty())
        .map(|str| str.parse().unwrap())
        .collect()
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    pub fn parse(line: &str) -> Result<Card, &str> {
        let re = Regex::new(r"Card \s*(\d+): ([\d\s]+) \| ([\d\s]+)").unwrap();

        if let Some(captures) = re.captures(line) {
            let winning_numbers = get_number_array(captures.get(2).unwrap().as_str());

            Ok(Self {
                id: captures.get(1).unwrap().as_str().trim().parse().unwrap(),
                winning_numbers: HashSet::from_iter(winning_numbers),
                numbers: get_number_array(captures.get(3).unwrap().as_str()),
            })
        } else {
            Err("Could not match")
        }
    }

    pub fn winning_count(&self) -> usize {
        self.numbers.iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count()
    }

    pub fn score(&self) -> u128 {
        let count = self.winning_count();
        if count == 0 {
            0
        } else {
            2_u128.pow((count - 1) as u32)
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let mut total_score: u128 = 0;
    let mut cards = vec![];

    while io::stdin().read_line(&mut buffer).unwrap() != 0 {
        let card = Card::parse(buffer.trim()).unwrap();

        total_score += card.score();
        cards.push((1, card));

        buffer.clear();
    }

    for index in 0..cards.len() {
        let count = cards[index].1.winning_count();

        for n in 0..count {
            let next_index = index + n + 1;

            if next_index < cards.len() {
                cards[index + n + 1].0 += cards[index].0;
            }
        }
    }

    let total_cards: u128 = cards.iter()
        .map(|(copies, _)| copies)
        .sum();

    println!("total score = {}", total_score);
    println!("total cards = {}", total_cards);
}

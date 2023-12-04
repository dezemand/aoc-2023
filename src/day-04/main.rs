use std::collections::{HashSet, VecDeque};
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
    let mut total_score = 0;
    let mut next_copies = VecDeque::with_capacity(5);
    let mut total_copies = 0;

    while io::stdin().read_line(&mut buffer).unwrap() != 0 {
        let card = Card::parse(buffer.trim()).unwrap();

        let current_copies = next_copies.pop_front()
            .or(Some(1))
            .unwrap();
        let count = card.winning_count();

        for n in 0..count {
            if let Some(queue_item) = next_copies.get_mut(n) {
                *queue_item += current_copies;
            } else {
                next_copies.push_back(current_copies + 1);
            }
        }

        total_copies += current_copies;
        total_score += card.score();

        buffer.clear();
    }

    println!("total score = {}", total_score);
    println!("total cards = {}", total_copies);
}

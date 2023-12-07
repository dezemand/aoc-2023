use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::io;

use itertools;
use itertools::Itertools;

fn make_histogram<T>(items: Vec<T>) -> Vec<(T, usize)>
    where T: Eq + Clone + Hash {
    let mut histogram: HashMap<T, usize> = HashMap::new();

    for item in items {
        *histogram.entry(item.clone()).or_insert(0) += 1;
    }

    histogram.into_iter()
        .sorted_by(|(_, a), (_, b)| b.cmp(a))
        .collect()
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Card(char);

impl Card {
    pub fn new(card_type: char) -> Self { Self(card_type) }

    pub fn strength(&self) -> u32 {
        match self.0 {
            digit @ '2'..='9' => digit.to_digit(10).unwrap(),
            'J' => 1,
            'T' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0
        }
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

const JOKER: Card = Card('J');

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    histogram: Vec<(Card, usize)>,
    jokers: usize,
}

impl Hand {
    pub fn parse(str: &str) -> Self {
        let cards: [Card; 5] = str.chars()
            .map(|char| Card::new(char))
            .collect_tuple::<(Card, Card, Card, Card, Card)>()
            .unwrap()
            .try_into()
            .unwrap();

        let histogram = make_histogram(cards
            .iter()
            .map(|card| *card)
            .filter(|card| *card != JOKER)
            .collect()
        );

        Self {
            cards,
            histogram,
            jokers: cards.iter().filter(|card| **card == JOKER).count(),
        }
    }

    pub fn nth_combination(&self, rank: usize) -> Option<&(Card, usize)> {
        if self.histogram.len() > rank { Some(&self.histogram[rank]) } else { None }
    }

    pub fn rank(&self) -> u32 {
        let top = self.nth_combination(0).map(|(_, rank)| *rank).unwrap_or_else(|| 0)
            + self.jokers;

        match top {
            5 => 7,
            4 => 6,
            3 => match self.nth_combination(1) {
                Some((_, 2)) => 5,
                _ => 4,
            },
            2 => match self.nth_combination(1) {
                Some((_, 2)) => 3,
                _ => 2,
            },
            1 => 1,
            _ => 0
        }
    }

    pub fn cmp_cards(&self, other: &Self) -> Ordering {
        for index in 0..5 {
            if self.cards[index] != other.cards[index] {
                return self.cards[index].cmp(&other.cards[index]);
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank().cmp(&other.rank()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cmp_cards(other),
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let mut hands: Vec<(Hand, u32)> = Vec::new();

    while io::stdin().read_line(&mut buffer).unwrap() != 0 {
        let (hand, bid) = buffer.split_whitespace()
            .collect_tuple().unwrap();

        hands.push((Hand::parse(hand), bid.parse().unwrap()));

        buffer.clear();
    }

    let total_score: u128 = hands.iter()
        .sorted()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) as u128 * (*bid as u128))
        .sum();

    println!("total score = {}", total_score);
}

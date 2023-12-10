use std::io;
use crate::sequence::Sequence;

mod sequence;

fn parse_input() -> Vec<Sequence> {
    let mut buffer = String::new();
    let mut sequences = Vec::new();

    while io::stdin().read_line(&mut buffer).unwrap() != 0 {
        let numbers: Vec<i64> = buffer.trim().split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        if !numbers.is_empty() {
            sequences.push(Sequence::new(numbers));
        }

        buffer.clear();
    }

    sequences
}

fn get_differences_list(sequence: &Sequence) -> Vec<Sequence> {
    let mut differences = vec![sequence.clone()];

    while !differences.last().unwrap().is_zeros() {
        differences.push(differences.last().unwrap().get_difference_sequence());
    }

    differences
}

fn get_next_number(differences: &Vec<Sequence>) -> i64 {
    let steps = differences.len() - 1;
    let mut next_number = 0;

    for step in 0..steps {
        let difference_index = steps - 1 - step;
        let last = differences[difference_index].last();
        next_number = *last + next_number;
    }

    next_number
}

fn get_previous_number(differences: &Vec<Sequence>) -> i64 {
    let steps = differences.len() - 1;
    let mut next_number = 0;

    for step in 0..steps {
        let difference_index = steps - 1 - step;
        let first = differences[difference_index].first();
        next_number = *first - next_number;
    }

    next_number
}

fn main() {
    let sequences = parse_input();

    let diff_sequences: Vec<Vec<Sequence>> = sequences.iter()
        .map(|sequence| get_differences_list(sequence))
        .collect();

    let total_next: i64 = diff_sequences.iter()
        .map(|sequence| get_next_number(sequence))
        .sum();

    let total_previous: i64 = diff_sequences.iter()
        .map(|sequence| get_previous_number(sequence))
        .sum();

    println!("total next     = {}", total_next);
    println!("total previous = {}", total_previous);
}

use std::collections::BTreeSet;
use std::io;

use regex::Regex;

use crate::seeds::seeds::{SeedMapping, SeedMaps, SeedRange};

mod seeds;

fn parse_seeds(line: &str) -> (Vec<u32>, Vec<SeedRange>) {
    let new_seeds: Vec<u32> = line.split_whitespace()
        .map(|str| str.parse().unwrap())
        .collect();
    let mut seed_pairs = vec![];

    for i in (0..new_seeds.len()).step_by(2) {
        seed_pairs.push(SeedRange::new(new_seeds[i], new_seeds[i + 1]));
    }

    (new_seeds, seed_pairs)
}

fn parse_input() -> (Vec<u32>, Vec<SeedRange>, SeedMaps) {
    let re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();

    let mut buffer = String::new();
    let mut seeds: Vec<u32> = vec![];
    let mut seed_pairs: Vec<SeedRange> = vec![];
    let mut seed_maps = SeedMaps::new();

    let mut current_map_key: Option<String> = None;
    let mut start_of_section = true;

    while io::stdin().read_line(&mut buffer).unwrap() != 0 {
        if !start_of_section && buffer.trim().is_empty() {
            start_of_section = true;
            buffer.clear();
            continue;
        }

        if start_of_section {
            if buffer.starts_with("seeds: ") {
                let (singles, pairs) = parse_seeds(buffer.strip_prefix("seeds: ").unwrap());
                seeds.extend(singles);
                seed_pairs.extend(pairs);
            } else {
                let captures = re.captures(buffer.as_str()).unwrap();

                let source = captures.get(1).unwrap().as_str();
                let destination = captures.get(2).unwrap().as_str();

                let key = seed_maps.new_map(source, destination);

                current_map_key = Some(key);
            }

            start_of_section = false;
        } else {
            if let Some(map_key) = &current_map_key {
                seed_maps.get_mut_map(map_key)
                    .add_mapping(SeedMapping::parse(buffer.as_str()));
            } else {
                let (singles, pairs) = parse_seeds(buffer.as_str());
                seeds.extend(singles);
                seed_pairs.extend(pairs);
            }
        }

        buffer.clear();
    }

    (seeds, seed_pairs, seed_maps)
}

fn main() {
    let (seeds, seed_pairs, seed_maps) = parse_input();

    let min_location: u32 = seeds.iter()
        .map(|seed| seed_maps.get_value(*seed, "seed", "location").unwrap())
        .min()
        .unwrap();

    let ranges = seed_maps.get_value_for_ranges(
        BTreeSet::from_iter(seed_pairs),
        "seed",
        "location"
    )
        .unwrap();

    println!("single seed min location = {}", min_location);
    println!("ranges seed min location = {}", ranges.first().unwrap().lowest());
}

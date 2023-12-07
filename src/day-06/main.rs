use std::io;

fn parse_input_strings() -> (String, String) {
    let mut time_str = String::new();
    let mut distance_str = String::new();

    let io_result = (
        io::stdin().read_line(&mut time_str).unwrap(),
        io::stdin().read_line(&mut distance_str).unwrap()
    );

    if io_result.0 == 0 || io_result.1 == 0 {
        panic!("Could not read input");
    }

    (
        time_str.strip_prefix("Time:").unwrap().trim().to_string(),
        distance_str.strip_prefix("Distance:").unwrap().trim().to_string()
    )
}

fn parse_races((time_str, distance_str): (&str, &str)) -> Vec<(u64, u64)> {
    time_str
        .split_whitespace()
        .map(|time| time.parse().unwrap())
        .zip(distance_str.split_whitespace()
            .map(|time| time.parse().unwrap())
        )
        .collect()
}

fn parse_race((time_str, distance_str): (&str, &str)) -> (u64, u64) {
    (
        time_str.replace(" ", "").parse().unwrap(),
        distance_str.replace(" ", "").parse().unwrap(),
    )
}

fn number_of_possible_wins((max_time, required_distance): &(u64, u64)) -> u32 {
    println!("Race\tmax_time={}\trequired_distance={}", max_time, required_distance);

    let a: f64 = -1f64;
    let b: f64 = *max_time as f64;
    let c: f64 = *required_distance as f64 * -1f64;

    let d: f64 = b.powf(2f64) - (4f64 * a * c);

    if d == 0f64 {
        1
    } else if d < 0f64 {
        0
    } else {
        let x1 = ((b * -1f64 + d.sqrt()) / 2f64 * a + 0.00001).ceil() as u32;
        let x2 = ((b * -1f64 - d.sqrt()) / 2f64 * a - 0.00001).floor() as u32;

        x2 - x1 + 1
    }
}

fn main() {
    let input = parse_input_strings();
    let input_ptr = (input.0.as_str(), input.1.as_str());

    let races = parse_races(input_ptr);
    let big_race = parse_race(input_ptr);

    let races_result: u128 = races.iter()
        .map(|race| number_of_possible_wins(race) as u128)
        .product();

    let race_result = number_of_possible_wins(&big_race);

    println!("small races = {}", races_result);
    println!("big race    = {}", race_result);
}

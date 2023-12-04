use std::io;

static DIGITS: [(&str, u32); 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9)
];

fn find_digit(chars: Vec<char>, from_right: bool) -> u32 {
    let max = chars.len();

    let mut offset = 0;
    let mut result: Option<u32> = None;

    while offset < max && result.is_none() {
        if chars[offset].is_ascii_digit() {
            result = Some(chars[offset].to_digit(10).unwrap());
            break
        }

        for (name, digit) in DIGITS {
            if (offset + 1) < name.len() {
                continue
            }

            let subset = &chars[((offset + 1) - name.len())..=offset];

            let string = if from_right {
                String::from_iter(subset.iter().rev())
            } else {
                String::from_iter(subset)
            };

            if string == name {
                result = Some(digit);
                break
            }
        }

        offset += 1;
    }

    result.expect("Couldn't find a digit")
}

fn main() {
    let mut buffer = String::new();
    let mut total: u128 = 0;

    while io::stdin().read_line(&mut buffer).unwrap() != 0 {
        let first_digit = find_digit(buffer.trim().chars().collect(), false);
        let last_digit = find_digit(buffer.trim().chars().rev().collect(), true);

        let number = first_digit * 10 + last_digit;

        total += number as u128;

        buffer.clear();
    }

    println!("total = {}", total);
}

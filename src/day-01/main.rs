use std::io;

static ARR: [(&str, u8); 10] = [
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

fn main() {
    let mut buffer = String::new();
    let mut total: u128 = 0;

    while io::stdin().read_line(&mut buffer).unwrap() != 0 {
        println!("Before {:?}", buffer);

        for (str, num) in ARR {

            buffer = buffer.replace(str, num.to_string().as_ref());
        }

        println!("After {:?}", buffer);

        let digits: Vec<u8> = buffer.as_bytes().iter()
            .filter(|char| char.is_ascii_digit())
            .map(|char| *char - '0' as u8)
            .collect();

        println!("Digits {:?}", digits);

        // 29, 83, 13, 24, 42, 14, and 76

        let first = digits.first().unwrap();
        let last = digits.last().unwrap();

        let number = *first * 10 + *last;

        total += number as u128;

        buffer.clear();
    }

    println!("{}", total);
}

use std::collections::HashSet;
use std::io;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position(i32, i32);

#[derive(Debug, Copy, Clone)]
enum Direction { Left, Right }

impl Position {
    pub fn go(&self, direction: Direction) -> Position {
        match direction {
            Direction::Left => Position(self.0 - 1, self.1),
            Direction::Right => Position(self.0 + 1, self.1)
        }
    }
}

fn get_surrounding_positions(position: Position, width: usize, height: usize) -> Vec<Position> {
    let mut positions = vec![];

    let start_x = position.0 - 1;
    let stop_x = position.0 + width as i32;
    let start_y = position.1 - 1;
    let stop_y = position.1 + height as i32;

    for x in start_x..=stop_x {
        positions.push(Position(x, start_y));
        positions.push(Position(x, stop_y));
    }

    for y in (start_y + 1)..stop_y {
        positions.push(Position(start_x, y));
        positions.push(Position(stop_x, y));
    }

    positions
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct EngineSchematicNumber {
    number: u16,
    position: Position,
    width: usize,
    height: usize,
}

impl EngineSchematicNumber {
    pub fn new(number_data: &str, last_x: i32, last_y: i32) -> Self {
        EngineSchematicNumber {
            number: number_data.parse().unwrap(),
            position: Position(
                last_x - number_data.len() as i32,
                last_y,
            ),
            width: number_data.len(),
            height: 1,
        }
    }
}

#[derive(Debug)]
struct EngineSchematic<'a> {
    width: usize,
    height: usize,
    schematic: &'a Vec<Vec<char>>,
}

impl<'a> EngineSchematic<'a> {
    pub fn new(schematic: &'a Vec<Vec<char>>) -> Self {
        if schematic.len() == 0 {
            panic!("Invalid schematic length");
        }

        EngineSchematic {
            schematic,
            width: schematic[0].len(),
            height: schematic.len(),
        }
    }

    fn get_char(&self, pos: &Position) -> char {
        self.schematic[pos.1 as usize][pos.0 as usize]
    }

    fn is_in_bounds(&self, pos: &Position) -> bool {
        pos.0 >= 0
            && pos.0 < self.width as i32
            && pos.1 >= 0
            && pos.1 < self.height as i32
    }

    pub fn get_gears(&self) -> Vec<Position> {
        let mut parts = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position(x as i32, y as i32);
                let char = self.get_char(&position);

                if char == '*' {
                    parts.push(position);
                }
            }
        }

        parts
    }

    pub fn get_numbers(&self) -> Vec<EngineSchematicNumber> {
        let mut numbers = Vec::new();
        let mut number = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.schematic[y][x].is_ascii_digit() {
                    number.push(self.schematic[y][x]);
                } else if !number.is_empty() {
                    numbers.push(EngineSchematicNumber::new(number.as_ref(), x as i32, y as i32));
                    number.clear();
                }
            }

            if !number.is_empty() {
                numbers.push(EngineSchematicNumber::new(number.as_ref(), self.width as i32, y as i32));
                number.clear();
            }
        }

        numbers
    }

    pub fn get_surrounding_parts(&self, number: &EngineSchematicNumber) -> Vec<(Position, char)> {
        let positions = get_surrounding_positions(number.position, number.width, number.height);

        positions
            .iter()
            .filter(|position| self.is_in_bounds(position))
            .map(|position| (position, self.get_char(position)))
            .filter(|(_, char)| !char.is_ascii_digit() && *char != '.')
            .map(|(position, char)| (position.clone(), char))
            .collect()
    }

    pub fn get_gear_ratio(&self, position: &Position) -> Option<u128> {
        let positions = get_surrounding_positions(position.clone(), 1, 1);

        let surrounding: HashSet<EngineSchematicNumber> = positions
            .iter()
            .filter(|position| self.is_in_bounds(position))
            .map(|position| self.find_number(position))
            .filter(|number| number.is_some())
            .map(|number| number.unwrap())
            .collect();

        if surrounding.len() == 2 {
            Some(surrounding.iter()
                .map(|number| number.number as u128)
                .product()
            )
        } else {
            None
        }
    }

    fn find_number(&self, position: &Position) -> Option<EngineSchematicNumber> {
        let starting_char = self.get_char(position);

        if !starting_char.is_ascii_digit() {
            return None;
        }

        let mut characters = vec![starting_char];
        let mut left_pos = *position;
        loop {
            left_pos = left_pos.go(Direction::Left);
            if !self.is_in_bounds(&left_pos) {
                break;
            }

            let char = self.get_char(&left_pos);
            if char.is_ascii_digit() {
                characters.insert(0, char);
            } else {
                break;
            }
        }

        let mut right_pos = *position;
        loop {
            right_pos = right_pos.go(Direction::Right);
            if !self.is_in_bounds(&right_pos) {
                break;
            }

            let char = self.get_char(&right_pos);
            if char.is_ascii_digit() {
                characters.push(char);
            } else {
                break;
            }
        }

        Some(EngineSchematicNumber::new(
            &String::from_iter(characters),
            left_pos.0,
            left_pos.1,
        ))
    }
}


fn main() {
    let mut buffer = String::new();
    let mut schematic_data: Vec<Vec<char>> = vec![];

    while io::stdin().read_line(&mut buffer).unwrap() != 0 {
        let line: Vec<char> = buffer.trim().chars().collect();
        schematic_data.push(line);
        buffer.clear();
    }

    let schematic = EngineSchematic::new(&schematic_data);
    let numbers = schematic.get_numbers();
    let gear_positions = schematic.get_gears();

    let sum_part_numbers: u128 = numbers
        .iter()
        .filter(|number| !schematic.get_surrounding_parts(number).is_empty())
        .map(|number| number.number as u128)
        .sum();

    let sum_gear_ratios: u128 = gear_positions
        .iter()
        .map(|position| schematic.get_gear_ratio(position))
        .filter(|ratio| ratio.is_some())
        .map(|ratio| ratio.unwrap())
        .sum();

    println!("sum part numbers = {}", sum_part_numbers);
    println!("sum gear ratios  = {}", sum_gear_ratios);
}

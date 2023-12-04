use std::io;

#[derive(Debug, Copy, Clone)]
struct Position(i32, i32);

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

#[derive(Debug)]
struct EngineSchematicPart {
    part: char,
    position: Position,
}

impl EngineSchematicPart {
    pub fn new(part: char, x: i32, y: i32) -> Self {
        EngineSchematicPart {
            part,
            position: Position(x, y)
        }
    }
}

#[derive(Debug)]
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

    pub fn get_gear_ratio(&self, _: &Position) -> Option<u128> {
        // let positions = get_surrounding_positions(position.clone(), 1, 1);

        // positions
        //     .iter()
        //     .filter(|position| self.is_in_bounds(position))
        //     .map(|position| )

        Some(1)
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

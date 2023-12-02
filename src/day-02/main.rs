use std::io;

use regex::Regex;

use crate::GameError::{InvalidGameId, NoGameId};

#[derive(Debug)]
struct BagConfiguration {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

#[derive(Debug)]
struct Game {
    id: u16,
    max_red: u16,
    max_green: u16,
    max_blue: u16,
}

#[derive(Debug)]
enum GameError {
    NoGameId,
    InvalidGameId,
}

impl Game {
    pub fn parse(str: &str) -> Result<Game, GameError> {
        let game_split: Vec<&str> = str.split(": ").collect();

        let game_id = game_split[0].strip_prefix("Game ")
            .ok_or_else(|| NoGameId)?
            .parse::<u16>().map_err(|_| InvalidGameId)?;

        let mut game = Game {
            id: game_id,
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        };

        Regex::new(r"([,;]) ").unwrap()
            .split(game_split[1].trim())
            .map(|item| item.split(" ").collect::<Vec<&str>>())
            .map(|vec| (vec[0].parse::<u16>().unwrap(), vec[1]))
            .collect::<Vec<(u16, &str)>>()
            .iter()
            .for_each(|(amount, cube_type)| game.add_hand(*amount, cube_type));

        Ok(game)
    }

    fn add_hand(&mut self, amount: u16, cube_type: &str) {
        match cube_type {
            "red" => self.max_red = self.max_red.max(amount),
            "green" => self.max_green = self.max_green.max(amount),
            "blue" => self.max_blue = self.max_blue.max(amount),
            _ => ()
        };
    }

    pub fn is_possible(&self, configuration: &BagConfiguration) -> bool {
        self.max_red <= configuration.red
            && self.max_blue <= configuration.blue
            && self.max_green <= configuration.green
    }

    pub fn power(&self) -> u128 {
        (self.max_blue as u128) * (self.max_green as u128) * (self.max_red as u128)
    }
}

fn main() {
    let configuration = BagConfiguration {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut total_possible: u16 = 0;
    let mut total_power: u128 = 0;

    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).unwrap() != 0 {
        let game = Game::parse(buffer.as_ref()).unwrap();

        if game.is_possible(&configuration) {
            total_possible += game.id;
        }

        total_power += game.power();

        buffer.clear();
    }

    println!("total possible = {}", total_possible);
    println!("total power = {}", total_power);
}

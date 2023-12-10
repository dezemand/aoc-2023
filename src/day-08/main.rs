use std::collections::HashMap;
use std::io;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct NodeId([char; 3]);

impl NodeId {
    pub fn new(str: &str) -> Self {
        Self(str
            .chars()
            .collect_tuple::<(char, char, char)>()
            .unwrap()
            .try_into()
            .unwrap()
        )
    }

    pub fn is_start(&self) -> bool {
        self.0[2] == 'A'
    }

    pub fn is_finish(&self) -> bool {
        self.0[2] == 'Z'
    }
}

const START: NodeId = NodeId(['A'; 3]);
const FINISH: NodeId = NodeId(['Z'; 3]);

fn parse_directions() -> Vec<Direction> {
    let mut buffer = String::new();

    if io::stdin().read_line(&mut buffer).unwrap() == 0 {
        panic!("Error reading line")
    }

    let directions = buffer.trim()
        .chars()
        .map(|char| match char {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid character {char}")
        })
        .collect();

    if io::stdin().read_line(&mut buffer).unwrap() == 0 {
        panic!("Error reading line")
    }

    directions
}

fn parse_nodes() -> HashMap<NodeId, (NodeId, NodeId)> {
    let mut directions: HashMap<NodeId, (NodeId, NodeId)> = HashMap::new();
    let mut buffer = String::new();
    let re = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    while io::stdin().read_line(&mut buffer).unwrap() != 0 {
        let captures = re.captures(buffer.as_str()).unwrap();

        let start = NodeId::new(captures.get(1).unwrap().as_str());
        let left = NodeId::new(captures.get(2).unwrap().as_str());
        let right = NodeId::new(captures.get(3).unwrap().as_str());

        directions.insert(start, (left, right));

        buffer.clear();
    }

    directions
}

fn parse_input() -> (Vec<Direction>, HashMap<NodeId, (NodeId, NodeId)>) {
    (
        parse_directions(),
        parse_nodes()
    )
}

fn main() {
    let (directions, nodes) = parse_input();

    let mut current_node = &START;
    let mut steps: u128 = 0;

    while *current_node != FINISH {
        let direction_index = (steps % directions.len() as u128) as usize;
        let direction = directions.get(direction_index).unwrap();
        let (left_node, right_node) = nodes.get(current_node).unwrap();

        current_node = match direction {
            Direction::Left => left_node,
            Direction::Right => right_node
        };

        steps += 1;
    }

    println!("total steps AAA to ZZZ = {}", steps);


    let mut current_nodes: Vec<&NodeId> = nodes.keys()
        .filter(|key| key.is_start())
        .collect();

    println!("Starting nodes: {:?}", current_nodes);

    while current_nodes.iter().any(|node| !node.is_finish()) {
        let direction_index = (steps % directions.len() as u128) as usize;
        let direction = directions.get(direction_index).unwrap();
        let (left_node, right_node) = nodes.get(current_node).unwrap();

        for node in current_nodes.as_mut_slice() {
            *node = match direction {
                Direction::Left => left_node,
                Direction::Right => right_node
            }
        }

        steps += 1;
    }

    println!("total steps xxA to xxZ in {} nodes = {}", current_nodes.len(), steps);
}

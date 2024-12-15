use std::{collections::HashMap, fs};

use crate::day8::Position;

struct Config {
    in_file: String,
}

impl Config {
    fn new(args: &mut impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let in_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing input file argument"),
        };

        Ok(Config { in_file })
    }
}

struct Map {
    robot: Position,
    map: HashMap<Position, bool>,
}

impl Map {
    fn new(raw_map_dataset: &str) -> Self {
        let size = raw_map_dataset.find("\n").unwrap();
        let robot_offset = raw_map_dataset.find("@").unwrap();
        let robot = Position {
            col: (robot_offset % size) as u8,
            row: (robot_offset / size) as u8,
        };
        let lines = raw_map_dataset.lines();

        let map: HashMap<Position, bool> = lines
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(move |(col, c)| {
                    let position = Position {
                        row: row as u8,
                        col: col as u8,
                    };
                    (position, c)
                })
            })
            .filter_map(|(position, c)| match c {
                '#' => Some((position, false)),
                'O' => Some((position, true)),
                _ => None,
            })
            .fold(HashMap::new(), |mut acc, (position, movable)| {
                acc.insert(position, movable);
                acc
            });

        Map { robot, map }
    }

    fn clear_position(&mut self, position: &Position, direction: &char) -> bool {
        let current = self.map.get(position);
        match current {
            None => true,
            Some(&false) => false,
            Some(&true) => {
                let next_position = match direction {
                    '>' => Position {
                        row: position.row,
                        col: position.col + 1,
                    },
                    '^' => Position {
                        row: position.row - 1,
                        col: position.col,
                    },
                    'v' => Position {
                        row: position.row + 1,
                        col: position.col,
                    },
                    '<' => Position {
                        row: position.row,
                        col: position.col - 1,
                    },
                    _ => panic!("Invalid direction"),
                };
                if self.clear_position(&next_position, direction) {
                    // self.map.insert(*position, true);
                    self.map.insert(next_position, true);
                    self.map.remove(position);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn move_robot(&mut self, direction: &char) {
        let next_position = match direction {
            '>' => Position {
                row: self.robot.row,
                col: self.robot.col + 1,
            },
            '^' => Position {
                row: self.robot.row - 1,
                col: self.robot.col,
            },
            'v' => Position {
                row: self.robot.row + 1,
                col: self.robot.col,
            },
            '<' => Position {
                row: self.robot.row,
                col: self.robot.col - 1,
            },
            _ => panic!("Invalid direction"),
        };
        if self.clear_position(&next_position, direction) {
            self.robot = next_position;
        }
    }

    fn get_box_sum_coordinate(&self) -> u32 {
        self.map
            .iter()
            .filter_map(|(position, movable)| match movable {
                true => Some((position.row as u32 * 100) + position.col as u32),
                false => None,
            })
            .sum()
    }
}

fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        std::process::exit(1);
    })
}

fn process_first(map: &mut Map, instructions: &str) -> u32 {
    for instruction in instructions.chars() {
        map.move_robot(&instruction);
    }
    map.get_box_sum_coordinate()
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);
    let mut raw_dataset_split = raw_dataset.split("\n\n");
    let raw_map_dataset = raw_dataset_split.next().unwrap();
    let raw_instruction_dataset = raw_dataset_split.next().unwrap().replace("\n", "");
    let mut map = Map::new(raw_map_dataset);
    let box_coor_sum = process_first(&mut map, &raw_instruction_dataset);
    println!("Sum of all boxes final coordinates: {}", box_coor_sum);
}

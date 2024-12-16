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

impl Position {
    fn next_position(&self, direction: &char) -> Position {
        match direction {
            '>' => Position {
                row: self.row,
                col: self.col + 1,
            },
            '^' => Position {
                row: self.row - 1,
                col: self.col,
            },
            'v' => Position {
                row: self.row + 1,
                col: self.col,
            },
            '<' => Position {
                row: self.row,
                col: self.col - 1,
            },
            _ => panic!("Invalid direction"),
        }
    }
}

struct Map {
    robot: Position,
    map: HashMap<Position, bool>,
}

impl Map {
    fn new(raw_map_dataset: &str) -> Self {
        let size = raw_map_dataset.find("\n").unwrap() + 1;
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
        let next_position = self.robot.next_position(direction);
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

struct MapDoubleWide {
    robot: Position,
    map: HashMap<Position, Option<bool>>,
}

impl MapDoubleWide {
    fn new(raw_map_dataset: &str) -> Self {
        let size = raw_map_dataset.find("\n").unwrap() + 1;
        let robot_offset = raw_map_dataset.find("@").unwrap();
        let robot = Position {
            col: (robot_offset % size) as u8 * 2,
            row: (robot_offset / size) as u8,
        };

        let lines = raw_map_dataset.lines();

        let map = lines
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(move |(col, c)| {
                    let position = Position {
                        row: row as u8,
                        col: col as u8 * 2,
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
                let position_on_right = Position {
                    row: position.row,
                    col: position.col + 1,
                };
                let movable_option = match movable {
                    true => Some(true),
                    false => None,
                };
                let moveable_on_right_option = match movable {
                    true => Some(false),
                    false => None,
                };
                acc.insert(position, movable_option);
                acc.insert(position_on_right, moveable_on_right_option);
                acc
            });

        MapDoubleWide { robot, map }
    }

    fn can_move(&mut self, position: &Position, direction: &char) -> bool {
        let current_position_data = self.map.get(position);
        let another_position_of_box = match current_position_data {
            None => {
                return true;
            }
            Some(movable) => match movable {
                None => {
                    return false;
                }
                Some(true) => Position {
                    row: position.row,
                    col: position.col + 1,
                },
                Some(false) => Position {
                    row: position.row,
                    col: position.col - 1,
                },
            },
        };
        let next_position = position.next_position(direction);
        let next_another_position_of_box = another_position_of_box.next_position(direction);
        let self_can_move = self.can_move(&next_position, direction);
        let another_can_move = next_another_position_of_box == *position
            || self.can_move(&next_another_position_of_box, direction);
        self_can_move && another_can_move
    }

    fn clear_position(&mut self, position: &Position, direction: &char) {
        let current_position_data = self.map.get(position);
        let another_position_of_box = match current_position_data {
            None => {
                return;
            }
            Some(movable) => match movable {
                None => {
                    return;
                }
                Some(true) => Position {
                    row: position.row,
                    col: position.col + 1,
                },
                Some(false) => Position {
                    row: position.row,
                    col: position.col - 1,
                },
            },
        };
        let current_position_data = self.map.remove(position).unwrap();
        let another_position_of_box_data = self.map.remove(&another_position_of_box).unwrap();
        let next_position = position.next_position(direction);
        let next_another_position_of_box = another_position_of_box.next_position(direction);
        self.clear_position(&next_position, direction);
        self.clear_position(&next_another_position_of_box, direction);
        self.map.insert(next_position, current_position_data);
        self.map
            .insert(next_another_position_of_box, another_position_of_box_data);
    }

    fn move_robot(&mut self, direction: &char) {
        let next_position = self.robot.next_position(direction);
        if self.can_move(&next_position, direction) {
            self.clear_position(&next_position, direction);
            self.robot = next_position;
        }
    }

    fn get_box_sum_coordinate(&self) -> u32 {
        self.map
            .iter()
            .filter_map(|(position, is_left)| match is_left {
                Some(true) => Some((position.row as u32 * 100) + position.col as u32),
                _ => None,
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

fn process_second(map: &mut MapDoubleWide, instructions: &str) -> u32 {
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
    let instructions = raw_dataset_split.next().unwrap().replace("\n", "");

    let mut map = Map::new(raw_map_dataset);
    let box_coor_sum = process_first(&mut map, &instructions);
    println!("Sum of all boxes final coordinates: {}", box_coor_sum);

    let mut map_double_wide = MapDoubleWide::new(raw_map_dataset);
    let box_coor_sum_double_wide = process_second(&mut map_double_wide, &instructions);
    println!(
        "Sum of all boxes final coordinates in second warehouse: {}",
        box_coor_sum_double_wide
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_ex_small() {
        let raw_dataset = read_input_file("input/day15_ex_small.txt");
        let mut raw_dataset_split = raw_dataset.split("\n\n");
        let raw_map_dataset = raw_dataset_split.next().unwrap();
        let instructions = raw_dataset_split.next().unwrap().replace("\n", "");

        let mut map = Map::new(raw_map_dataset);
        let box_coor_sum = process_first(&mut map, &instructions);
        assert_eq!(box_coor_sum, 2028);

        let mut map_double_wide = MapDoubleWide::new(raw_map_dataset);
        let box_coor_sum_double_wide = process_second(&mut map_double_wide, &instructions);
        assert_eq!(box_coor_sum_double_wide, 1751);
    }

    #[test]
    fn test_process_ex() {
        let raw_dataset = read_input_file("input/day15_ex.txt");
        let mut raw_dataset_split = raw_dataset.split("\n\n");
        let raw_map_dataset = raw_dataset_split.next().unwrap();
        let instructions = raw_dataset_split.next().unwrap().replace("\n", "");

        let mut map = Map::new(raw_map_dataset);
        let box_coor_sum = process_first(&mut map, &instructions);
        assert_eq!(box_coor_sum, 10092);

        let mut map_double_wide = MapDoubleWide::new(raw_map_dataset);
        let box_coor_sum_double_wide = process_second(&mut map_double_wide, &instructions);
        assert_eq!(box_coor_sum_double_wide, 9021);
    }

    #[test]
    fn test_process() {
        let raw_dataset = read_input_file("input/day15.txt");
        let mut raw_dataset_split = raw_dataset.split("\n\n");
        let raw_map_dataset = raw_dataset_split.next().unwrap();
        let instructions = raw_dataset_split.next().unwrap().replace("\n", "");

        let mut map = Map::new(raw_map_dataset);
        let box_coor_sum = process_first(&mut map, &instructions);
        assert_eq!(box_coor_sum, 1563092);

        let mut map_double_wide = MapDoubleWide::new(raw_map_dataset);
        let box_coor_sum_double_wide = process_second(&mut map_double_wide, &instructions);
        assert_eq!(box_coor_sum_double_wide, 1582688);
    }
}

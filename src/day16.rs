use std::{collections::{HashMap, HashSet}, fs};

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

fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        std::process::exit(1);
    })
}

struct Map {
    map: HashSet<Position>,
    start: Position,
    end: Position,
}

impl Map {
    fn new(raw_dataset: &str) -> Map {
        let farthest = raw_dataset.find("\n").unwrap() as u8 - 2;
        let start = Position {
            col: 1,
            row: farthest,
        };
        let end = Position {
            col: farthest,
            row: 1,
        };
        let map = raw_dataset
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(move |(col, c)| {
                    (
                        Position {
                            col: col as u8,
                            row: row as u8,
                        },
                        c,
                    )
                })
            })
            .filter_map(|(pos, c)| match c {
                '#' => Some(pos),
                _ => None,
            })
            .collect();
        Map { map, start, end }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
struct Robot {
    position: Position,
    facing: Direction,
    score: usize,
}

impl Robot {
    fn get_left_direction(&self) -> Direction {
        match self.facing {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn get_right_direction(&self) -> Direction {
        match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn get_next_with_direction(&self, direction: Direction) -> Result<Position, &'static str> {
        match direction {
            Direction::North => {
                let row = self.position.row.checked_sub(1).ok_or("Overflow")?;
                Ok(Position {
                    col: self.position.col,
                    row
                })
            },
            Direction::East => {
                let col = self.position.col.checked_add(1).ok_or("Overflow")?;
                Ok(Position {
                    col,
                    row: self.position.row
                })
            },
            Direction::South => {
                let row = self.position.row.checked_add(1).ok_or("Overflow")?;
                Ok(Position {
                    col: self.position.col,
                    row
                })
            },
            Direction::West => {
                let col = self.position.col.checked_sub(1).ok_or("Overflow")?;
                Ok(Position {
                    col,
                    row: self.position.row
                })
            },
        }
    }

    fn can_go_to(&self, pos: &Position, map: &Map) -> bool {
        !map.map.contains(pos)
    }

    fn can_forward(&self, map: &Map) -> bool {
        if let Ok(next) = self.get_next_with_direction(self.facing) {
            return self.can_go_to(&next, map);
        } else {
            false
        }
    }

    fn can_left(&self, map: &Map) -> bool {
        if let Ok(next) = self.get_next_with_direction(self.get_left_direction()) {
            return self.can_go_to(&next, map);
        } else {
            false
        }
    }

    fn can_right(&self, map: &Map) -> bool {
        if let Ok(next) = self.get_next_with_direction(self.get_right_direction()) {
            return self.can_go_to(&next, map);
        } else {
            false
        }
    }

    fn go_forward(&mut self) {
        if let Ok(next) = self.get_next_with_direction(self.facing) {
            self.position = next;
            self.score += 1;
        }
    }

    fn go_left(&mut self) {
        self.facing = self.get_left_direction();
        self.score += 1000;
        self.go_forward();
    }

    fn go_right(&mut self) {
        self.facing = self.get_right_direction();
        self.score += 1000;
        self.go_forward();
    }
}

fn process_first(map: &Map) -> usize {
    let mut robots = vec![Robot {
        position: map.start,
        facing: Direction::East,
        score: 0,
    }];
    let mut least_score = usize::MAX;
    let mut visited: HashMap<(Position, Direction), usize> = HashMap::new();
    loop {
        let least_score_robot_i = robots
            .iter()
            .enumerate()
            .min_by_key(|(_, robot)| robot.score)
            .unwrap()
            .0;
        let robot = robots.swap_remove(least_score_robot_i);
        if least_score < robot.score {
            return least_score;
        }
        if robot.can_forward(map) {
            let mut robot = robot.clone();
            robot.go_forward();
            if robot.position == map.end {
                least_score = robot.score;
            }
            match visited.get(&(robot.position, robot.facing)).map(|score| &robot.score < score) {
                Some(false) => (),
                _ => {
                    visited.insert((robot.position, robot.facing), robot.score);
                    robots.push(robot);
                }
            }
        }
        if robot.can_left(map) {
            let mut robot = robot.clone();
            robot.go_left();
            if robot.position == map.end {
                least_score = robot.score;
            }
            match visited.get(&(robot.position, robot.facing)).map(|score| &robot.score < score) {
                Some(false) => (),
                _ => {
                    visited.insert((robot.position, robot.facing), robot.score);
                    robots.push(robot);
                }
            }
        }
        if robot.can_right(map) {
            let mut robot = robot.clone();
            robot.go_right();
            if robot.position == map.end {
                least_score = robot.score;
            }
            match visited.get(&(robot.position, robot.facing)).map(|score| &robot.score < score) {
                Some(false) => (),
                _ => {
                    visited.insert((robot.position, robot.facing), robot.score);
                    robots.push(robot);
                }
            }
        }
    }
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);
    let map = Map::new(&raw_dataset);

    let result = process_first(&map);
    println!("Result: {}", result);
}

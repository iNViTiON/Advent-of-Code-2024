use std::{
    collections::{HashMap, HashSet},
    fs,
};

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

#[derive(Clone)]
enum Direction {
    Up,
    Down,
}

struct TrailPossibleFromSource {
    source: HashSet<Position>,
    direction: Direction,
    current_height: u8,
    current_position: Position,
}

fn can_step_to(maps: &Vec<Vec<u8>>, position: &Position, to_height: &u8) -> HashSet<Position> {
    [-1, 1]
        .into_iter()
        .flat_map(|offset| {
            let mut expand: Vec<Position> = Vec::new();
            let row = position.row as i8 + offset;
            if row >= 0 {
                expand.push(Position {
                    row: row as u8,
                    col: position.col,
                });
            };
            let col = position.col as i8 + offset;
            if col >= 0 {
                expand.push(Position {
                    row: position.row,
                    col: col as u8,
                });
            };
            expand
        })
        .filter_map(|position| {
            maps.get(position.row as usize)
                .and_then(|row| row.get(position.col as usize))
                .and_then(|height| {
                    if height == to_height {
                        Some(position)
                    } else {
                        None
                    }
                })
        })
        .collect()
}

fn step(
    maps: &Vec<Vec<u8>>,
    trail_path: &TrailPossibleFromSource,
) -> HashMap<Position, TrailPossibleFromSource> {
    let next_height = match trail_path.direction {
        Direction::Up => trail_path.current_height + 1,
        Direction::Down => trail_path.current_height - 1,
    };
    can_step_to(maps, &trail_path.current_position, &next_height)
        .into_iter()
        .fold(HashMap::new(), |mut acc, position| {
            acc.insert(
                position,
                TrailPossibleFromSource {
                    source: trail_path.source.clone(),
                    direction: trail_path.direction.clone(),
                    current_height: next_height,
                    current_position: position,
                },
            );
            acc
        })
}

fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        std::process::exit(1);
    })
}

fn get_positions_by_height(maps: &Vec<Vec<u8>>, target_height: u8) -> HashSet<Position> {
    maps.iter()
        .enumerate()
        .fold(HashSet::new(), |mut acc, (row_i, col)| {
            acc.extend(
                col.iter()
                    .enumerate()
                    .filter_map(|(col_i, &height)| match height {
                        height if target_height == height => Some(Position {
                            row: row_i as u8,
                            col: col_i as u8,
                        }),
                        _ => None,
                    }),
            );
            acc
        })
}

fn process_first(raw_dataset: &str) -> usize {
    let maps: Vec<Vec<u8>> = raw_dataset
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let trailheads = get_positions_by_height(&maps, 0);
    let trail_peaks = get_positions_by_height(&maps, 9);
    let mut head_paths: HashMap<Position, TrailPossibleFromSource> =
        trailheads
            .into_iter()
            .fold(HashMap::new(), |mut acc, trailhead| {
                acc.entry(trailhead)
                    .and_modify(|trail_path| {
                        trail_path.source.insert(trailhead);
                    })
                    .or_insert_with(|| {
                        let mut source = HashSet::new();
                        source.insert(trailhead);
                        TrailPossibleFromSource {
                            source,
                            direction: Direction::Up,
                            current_height: 0,
                            current_position: trailhead,
                        }
                    });
                acc
            });
    let mut peak_paths: HashMap<Position, TrailPossibleFromSource> =
        trail_peaks
            .into_iter()
            .fold(HashMap::new(), |mut acc, trailhead| {
                acc.entry(trailhead)
                    .and_modify(|trail_path| {
                        trail_path.source.insert(trailhead);
                    })
                    .or_insert_with(|| {
                        let mut source = HashSet::new();
                        source.insert(trailhead);
                        TrailPossibleFromSource {
                            source,
                            direction: Direction::Down,
                            current_height: 9,
                            current_position: trailhead,
                        }
                    });
                acc
            });
    for _ in 0..5 {
        head_paths = head_paths
            .into_iter()
            .fold(HashMap::new(), |mut acc, (_, trail_path)| {
                step(&maps, &trail_path)
                    .into_iter()
                    .for_each(|(position, trail_path)| {
                        acc.entry(position)
                            .and_modify(|exist_trail_path| {
                                exist_trail_path.source.extend(&trail_path.source)
                            })
                            .or_insert(trail_path);
                    });
                acc
            });
    }
    for _ in 0..4 {
        peak_paths = peak_paths
            .into_iter()
            .fold(HashMap::new(), |mut acc, (_, trail_path)| {
                step(&maps, &trail_path)
                    .into_iter()
                    .for_each(|(position, trail_path)| {
                        acc.entry(position)
                            .and_modify(|exist_trail_path| {
                                exist_trail_path.source.extend(&trail_path.source)
                            })
                            .or_insert(trail_path);
                    });
                acc
            });
    }

    let relations: HashMap<Position, HashSet<Position>> =
        head_paths
            .into_iter()
            .fold(HashMap::new(), |mut acc, (position, headtrail_path)| {
                let peak_positions = peak_paths
                    .get(&position)
                    .and_then(|peak_trail_path| Some(&peak_trail_path.source));
                if let Some(peak_positions) = peak_positions {
                    headtrail_path
                        .source
                        .into_iter()
                        .for_each(|headtrail_position| {
                            acc.entry(headtrail_position)
                                .and_modify(|exist_peak_positions| {
                                    exist_peak_positions.extend(peak_positions);
                                }).or_insert(peak_positions.clone());
                        });
                }
                acc
            });
    relations.into_iter().map(|(_, peak_positions)| peak_positions.len()).sum()
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);

    let scores_sum = process_first(&raw_dataset);
    println!("Sum of the scores of all trailheads: {}", scores_sum);
}

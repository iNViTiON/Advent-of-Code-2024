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
    source: HashMap<Position, u8>,
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

fn steps(
    maps: &Vec<Vec<u8>>,
    trail_paths: &HashMap<Position, TrailPossibleFromSource>,
) -> HashMap<Position, TrailPossibleFromSource> {
    trail_paths
        .into_iter()
        .fold(HashMap::new(), |mut acc, (_, trail_path)| {
            step(maps, trail_path)
                .into_iter()
                .for_each(|(position, trail_path)| {
                    acc.entry(position)
                        .and_modify(|exist_trail_path| {
                            trail_path.source.iter().for_each(
                                |(source_position_in_new_step, path_count)| {
                                    exist_trail_path
                                        .source
                                        .entry(*source_position_in_new_step)
                                        .and_modify(|exist_path_count| {
                                            *exist_path_count += path_count;
                                        })
                                        .or_insert(*path_count);
                                },
                            )
                        })
                        .or_insert(trail_path);
                });
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

fn process(raw_dataset: &str) -> HashMap<Position, HashMap<Position, u8>> {
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
    let mut head_paths: HashMap<Position, TrailPossibleFromSource> = trailheads
        .into_iter()
        .map(|position| {
            let mut source = HashMap::new();
            source.insert(position, 1);
            (
                position,
                TrailPossibleFromSource {
                    source,
                    direction: Direction::Up,
                    current_height: 0,
                    current_position: position,
                },
            )
        })
        .collect();
    let mut peak_paths: HashMap<Position, TrailPossibleFromSource> = trail_peaks
        .into_iter()
        .map(|position| {
            let mut source = HashMap::new();
            source.insert(position, 1);
            (
                position,
                TrailPossibleFromSource {
                    source,
                    direction: Direction::Down,
                    current_height: 9,
                    current_position: position,
                },
            )
        })
        .collect();
    for _ in 0..4 {
        head_paths = steps(&maps, &head_paths);
        peak_paths = steps(&maps, &peak_paths);
    }
    head_paths = steps(&maps, &head_paths);

    let relations: HashMap<Position, HashMap<Position, u8>> = head_paths
        .into_iter()
        .filter(|(position, _)| peak_paths.contains_key(position))
        .fold(HashMap::new(), |mut acc, (position, headtrail_path)| {
            let peak_positions = peak_paths
                .get(&position)
                .and_then(|peak_trail_path| Some(&peak_trail_path.source))
                // we already filter out the head_paths that are not in peak_paths
                .unwrap();
            headtrail_path.source.into_iter().for_each(
                |(headtrail_position, path_count_from_head)| {
                    acc.entry(headtrail_position)
                        .and_modify(|exist_peak_positions| {
                            peak_positions.iter().for_each(
                                |(peak_position, path_to_peak_count)| {
                                    exist_peak_positions
                                        .entry(*peak_position)
                                        .and_modify(|exist_path_count| {
                                            *exist_path_count +=
                                                path_to_peak_count * path_count_from_head;
                                        })
                                        .or_insert(*path_to_peak_count * path_count_from_head);
                                },
                            );
                        })
                        .or_insert({
                            let mut peak_with_score = HashMap::new();
                            peak_positions.iter().for_each(
                                |(peak_position, path_to_peak_count)| {
                                    peak_with_score.insert(
                                        *peak_position,
                                        path_to_peak_count * path_count_from_head,
                                    );
                                },
                            );
                            peak_with_score
                        });
                },
            );
            acc
        });
    relations
}

fn process_first(relations: &HashMap<Position, HashMap<Position, u8>>) -> usize {
    relations
        .iter()
        .map(|(_, peak_positions)| peak_positions.len())
        .sum()
}

fn process_second(relations: &HashMap<Position, HashMap<Position, u8>>) -> usize {
    relations
        .iter()
        .map(|(_, peak_positions)| peak_positions.values().sum::<u8>() as usize)
        .sum()
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);

    let relations = process(&raw_dataset);

    let scores_sum = process_first(&relations);
    println!("Sum of the scores of all trailheads: {}", scores_sum);
    
    let total_paths = process_second(&relations);
    println!("Total paths from trailheads to trailpeaks: {}", total_paths);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_ex() {
        let raw_dataset = read_input_file("input/day10_ex.txt");
        let relations = process(&raw_dataset);
        assert_eq!(process_first(&relations), 36);
        assert_eq!(process_second(&relations), 81);
    }

    #[test]
    fn test_process() {
        let raw_dataset = read_input_file("input/day10.txt");
        let relations = process(&raw_dataset);
        assert_eq!(process_first(&relations), 489);
        assert_eq!(process_second(&relations), 1086);
    }
}

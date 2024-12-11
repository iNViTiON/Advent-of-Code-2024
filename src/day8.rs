use std::collections::{HashMap, HashSet};
use std::fs;

use crate::day6::MapSize;

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

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

type AntennaMap = HashMap<char, Vec<Position>>;

fn create_attenna_map(raw_dataset: &str) -> AntennaMap {
    let mut map = AntennaMap::new();
    for (row, line) in raw_dataset.lines().enumerate() {
        for (col, c) in line.chars().enumerate().filter(|(_, c)| c != &'.') {
            map.entry(c).or_insert(Vec::new()).push(Position {
                row: row as u8,
                col: col as u8,
            });
        }
    }
    map
}

fn to_position(
    position: &Position,
    y_diff: &u8,
    y_add: &bool,
    x_diff: &u8,
    x_add: &bool,
    map_size: &MapSize,
) -> Result<Position, &'static str> {
    let row = {
        if *y_add {
            position.row.checked_add(*y_diff)
        } else {
            position.row.checked_sub(*y_diff)
        }
    }.ok_or("Position is out of map")?;
    let col = {
        if *x_add {
            position.col.checked_add(*x_diff)
        } else {
            position.col.checked_sub(*x_diff)
        }
    }
    .ok_or("Position is out of map")?;
    if row >= map_size.height || col >= map_size.width {
        return Err("Position is out of map");
    }
    Ok(Position { row, col })
}

fn process_first(map_size: &MapSize, antenna_map: &AntennaMap) -> usize {
    let mut antinodes: HashSet<Position> = HashSet::new();
    for freq in antenna_map.keys() {
        let positions = antenna_map.get(freq).unwrap();
        for pos1 in positions.iter() {
            for pos2 in positions.iter().filter(|pos2| *pos2 != pos1) {
                let y_min_is_1 = pos1.row < pos2.row;
                let x_min_is_1 = pos1.col < pos2.col;
                let (y_min, y_max) = if y_min_is_1 {
                    (pos1.row, pos2.row)
                } else {
                    (pos2.row, pos1.row)
                };
                let (x_min, x_max) = if x_min_is_1 {
                    (pos1.col, pos2.col)
                } else {
                    (pos2.col, pos1.col)
                };
                let x_diff = x_max - x_min;
                let y_diff = y_max - y_min;
                let antinode1 = to_position(
                    &pos1,
                    &y_diff,
                    &!y_min_is_1,
                    &x_diff,
                    &!x_min_is_1,
                    &map_size,
                );
                if let Ok(antinode1) = antinode1 {
                    antinodes.insert(antinode1);
                }
                let antinode2 = to_position(
                    &pos2,
                    &y_diff,
                    &y_min_is_1,
                    &x_diff,
                    &x_min_is_1,
                    &map_size,
                );
                if let Ok(antinode2) = antinode2 {
                    antinodes.insert(antinode2);
                }
            }
        }
    }
    antinodes.len()
}

fn process_second(map_size: &MapSize, antenna_map: &AntennaMap) -> usize {
    let mut antinodes: HashSet<Position> = HashSet::new();
    for freq in antenna_map.keys() {
        let positions = antenna_map.get(freq).unwrap();
        for pos1 in positions.iter() {
            for pos2 in positions.iter().filter(|pos2| *pos2 != pos1) {
                let y_min_is_1 = pos1.row < pos2.row;
                let x_min_is_1 = pos1.col < pos2.col;
                let (y_min, y_max) = if y_min_is_1 {
                    (pos1.row, pos2.row)
                } else {
                    (pos2.row, pos1.row)
                };
                let (x_min, x_max) = if x_min_is_1 {
                    (pos1.col, pos2.col)
                } else {
                    (pos2.col, pos1.col)
                };
                let x_diff = x_max - x_min;
                let y_diff = y_max - y_min;
                let mut antinode1 = Ok(pos1.clone());
                while antinode1.is_ok() {
                    let antinode = antinode1.unwrap();
                    antinode1 = to_position(
                        &antinode,
                        &y_diff,
                        &!y_min_is_1,
                        &x_diff,
                        &!x_min_is_1,
                        &map_size,
                    );
                    antinodes.insert(antinode);
                }
                let mut antinode2 = Ok(pos2.clone());
                while antinode2.is_ok() {
                    let antinode = antinode2.unwrap();
                    antinode2 = to_position(
                        &antinode,
                        &y_diff,
                        &y_min_is_1,
                        &x_diff,
                        &x_min_is_1,
                        &map_size,
                    );
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len()
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);
    let map_size = MapSize::from_maps(&raw_dataset);
    let antenna_map = create_attenna_map(&raw_dataset);
    let unique_antinode_location_count = process_first(&map_size, &antenna_map);
    println!("Unique antinode location count: {}", unique_antinode_location_count);
    let unique_antinode_with_resonant_harmonics_location_count = process_second(&map_size, &antenna_map);
    println!("Unique antinode with resonant harmonics location count: {}", unique_antinode_with_resonant_harmonics_location_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_ex() {
        let raw_dataset = read_input_file("input/day8_ex.txt");
        let map_size = MapSize::from_maps(&raw_dataset);
        let antenna_map = create_attenna_map(&raw_dataset);
        let unique_antinode_location_count = process_first(&map_size, &antenna_map);
        assert_eq!(unique_antinode_location_count, 14);
        let unique_antinode_with_resonant_harmonics_location_count = process_second(&map_size, &antenna_map);
        assert_eq!(unique_antinode_with_resonant_harmonics_location_count, 34);
    }

    #[test]
    fn test_process() {
        let raw_dataset = read_input_file("input/day8.txt");
        let map_size = MapSize::from_maps(&raw_dataset);
        let antenna_map = create_attenna_map(&raw_dataset);
        let unique_antinode_location_count = process_first(&map_size, &antenna_map);
        assert_eq!(unique_antinode_location_count, 357);
        let unique_antinode_with_resonant_harmonics_location_count = process_second(&map_size, &antenna_map);
        assert_eq!(unique_antinode_with_resonant_harmonics_location_count, 1266);
    }
}

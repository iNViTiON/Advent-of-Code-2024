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

#[derive(Eq, Hash, PartialEq)]
struct Position {
    row: u8,
    col: u8,
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
    row: &u8,
    y_diff: &u8,
    y_add: &bool,
    col: &u8,
    x_diff: &u8,
    x_add: &bool,
    map_size: &MapSize,
) -> Result<Position, &'static str> {
    let row = {
        if *y_add {
            row.checked_add(*y_diff)
        } else {
            row.checked_sub(*y_diff)
        }
    }.ok_or("Position is out of map")?;
    let col = {
        if *x_add {
            col.checked_add(*x_diff)
        } else {
            col.checked_sub(*x_diff)
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
                    &pos1.row,
                    &y_diff,
                    &!y_min_is_1,
                    &pos1.col,
                    &x_diff,
                    &!x_min_is_1,
                    &map_size,
                );
                if let Ok(antinode1) = antinode1 {
                    antinodes.insert(antinode1);
                }
                let antinode2 = to_position(
                    &pos2.row,
                    &y_diff,
                    &y_min_is_1,
                    &pos2.col,
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
}

use std::collections::HashMap;
use std::fs;

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

#[derive(Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_char(facing: &char) -> Direction {
        match facing {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => panic!("Invalid direction"),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Clone)]
struct GuardPosition {
    facing: Direction,
    row: u8,
    col: u8,
}

impl GuardPosition {
    fn from_maps(raw_dataset: &str) -> Result<GuardPosition, &'static str> {
        // plus 1 for the newline character
        let line_length = raw_dataset.lines().next().unwrap().len() + 1;
        let guard_offset = match raw_dataset.find(|c| c == '^' || c == 'v' || c == '<' || c == '>')
        {
            Some(offset) => offset,
            None => return Err("No guard found"),
        };
        Ok(GuardPosition {
            facing: Direction::from_char(&raw_dataset.chars().nth(guard_offset).unwrap()),
            row: (guard_offset / line_length) as u8,
            col: (guard_offset % line_length) as u8,
        })
    }

    fn turn_right(&mut self) {
        self.facing = self.facing.turn_right();
    }
}

struct ObstacleHashMap {
    rows: HashMap<u8, Vec<u8>>,
    cols: HashMap<u8, Vec<u8>>,
}

impl ObstacleHashMap {
    fn from_maps(raw_dataset: &str) -> ObstacleHashMap {
        let mut rows = HashMap::new();
        let mut cols = HashMap::new();
        for (row_i, line) in raw_dataset
            .lines()
            .enumerate()
            .filter(|row| row.1.contains('#'))
        {
            let row = rows.entry(row_i as u8).or_insert_with(Vec::new);
            for (col_i, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
                row.push(col_i as u8);
                cols.entry(col_i as u8)
                    .or_insert_with(Vec::new)
                    .push(row_i as u8);
            }
        }
        for row in rows.values_mut() {
            row.sort();
        }
        for col in cols.values_mut() {
            col.sort();
        }
        ObstacleHashMap { rows, cols }
    }
}

struct MapSize {
    width: u8,
    height: u8,
}

impl MapSize {
    fn from_maps(raw_dataset: &str) -> MapSize {
        let width = raw_dataset.lines().next().unwrap().len() as u8;
        let height = raw_dataset.lines().count() as u8;
        MapSize { width, height }
    }
}

struct Path(u8, u8);

struct MovementRecords {
    rows: HashMap<u8, Vec<Path>>,
    cols: HashMap<u8, Vec<Path>>,
}

fn get_movement_records(
    map_size: &MapSize,
    obstacles: &ObstacleHashMap,
    guard: &mut GuardPosition,
) -> MovementRecords {
    let mut rows = HashMap::new();
    let mut cols = HashMap::new();
    let mut out_of_map = false;
    loop {
        match guard.facing {
            Direction::North => {
                let new_row = obstacles
                    .cols
                    .get(&guard.col)
                    .and_then(|current_col| current_col.iter().rfind(|row_i| **row_i < guard.row))
                    .map(|obstacle| *obstacle + 1)
                    .unwrap_or_else(|| {
                        out_of_map = true;
                        0
                    });
                cols.entry(guard.col)
                    .or_insert_with(Vec::new)
                    .push(Path(new_row, guard.row));
                guard.row = new_row;
            }
            Direction::East => {
                let new_col = obstacles
                    .rows
                    .get(&guard.row)
                    .and_then(|current_row| current_row.iter().find(|col_i| **col_i > guard.col))
                    .map(|obstacle| *obstacle - 1)
                    .unwrap_or_else(|| {
                        out_of_map = true;
                        0
                    });
                rows.entry(guard.row)
                    .or_insert_with(Vec::new)
                    .push(Path(guard.col, new_col));
                guard.col = new_col;
            }
            Direction::South => {
                let new_row = obstacles
                    .cols
                    .get(&guard.col)
                    .and_then(|current_col| current_col.iter().find(|row_i| **row_i > guard.row))
                    .map(|obstacle| *obstacle - 1)
                    .unwrap_or_else(|| {
                        out_of_map = true;
                        map_size.height - 1
                    });
                cols.entry(guard.col)
                    .or_insert_with(Vec::new)
                    .push(Path(guard.row, new_row));
                guard.row = new_row;
            }
            Direction::West => {
                let new_col = obstacles
                    .rows
                    .get(&guard.row)
                    .and_then(|current_row| current_row.iter().rfind(|col_i| **col_i < guard.col))
                    .map(|obstacle| *obstacle + 1)
                    .unwrap_or_else(|| {
                        out_of_map = true;
                        map_size.width - 1
                    });
                rows.entry(guard.row)
                    .or_insert_with(Vec::new)
                    .push(Path(new_col, guard.col));
                guard.col = new_col;
            }
        }

        if out_of_map {
            break;
        }
        guard.turn_right();
    }
    MovementRecords { rows, cols }
}

fn simplify_vec(map: &mut HashMap<u8, Vec<Path>>) {
    for (_, paths) in map.iter_mut() {
        paths.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut i = 0;
        while i < paths.len() - 1 {
            let a = &paths[i];
            let b = &paths[i + 1];
            if a.1 >= b.0 {
                paths.get_mut(i).unwrap().1 = b.1;
                paths.remove(i + 1);
            } else {
                i += 1;
            }
        }
     }
}

fn simplify_visited(movement_records: &mut MovementRecords) {
    simplify_vec(&mut movement_records.rows);
    simplify_vec(&mut movement_records.cols);
}

fn count_crossed(movement_records: &MovementRecords) -> usize {
    let mut count = 0;
    for (row_i, paths_row) in movement_records.rows.iter() {
        for path_row in paths_row {
            for (col_i, paths_col) in movement_records.cols.iter() {
                for path_col in paths_col {
                    if path_row.0 <= *col_i
                        && *col_i <= path_row.1
                        && path_col.0 <= *row_i
                        && *row_i <= path_col.1
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn count_visited(movement_records: &MovementRecords) -> usize {
    movement_records
        .rows
        .iter()
        .chain(movement_records.cols.iter())
        .fold(0, |acc_paths, (_, paths)| {
            acc_paths
                + paths.iter().fold(0, |acc_path, path| {
                    acc_path + (path.1 - path.0 + 1) as usize
                })
        })
}

fn process_first(raw_dataset: &str) -> usize {
    let map_size = MapSize::from_maps(&raw_dataset);
    let obstacles = ObstacleHashMap::from_maps(&raw_dataset);
    let mut guard = GuardPosition::from_maps(&raw_dataset).unwrap();
    let mut movement_records = get_movement_records(&map_size, &obstacles, &mut guard);
    simplify_visited(&mut movement_records);
    let crossed = count_crossed(&movement_records);
    let visited = count_visited(&movement_records);
    visited - crossed
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);

    let distinct_visit = process_first(&raw_dataset);
    println!("Distinct visit: {}", distinct_visit);
}

#[cfg(test)]
mod tests {
    use super::*;

    // let's do document test starting today

    // #[test]
    // fn test_process_first_ex() {
    //     let raw_dataset = read_input_file("input/day4_ex.txt");
    //     assert_eq!(process_first(&raw_dataset), 18);
    // }

    // #[test]
    // fn test_process_first() {
    //     let raw_dataset = read_input_file("input/day4.txt");
    //     assert_eq!(process_first(&raw_dataset), 2571);
    // }

    // #[test]
    // fn test_process_second_ex() {
    //     let raw_dataset = read_input_file("input/day4_ex.txt");
    //     assert_eq!(process_second(&raw_dataset), 9);
    // }

    // #[test]
    // fn test_process_second() {
    //     let raw_dataset = read_input_file("input/day4.txt");
    //     assert_eq!(process_second(&raw_dataset), 1992);
    // }
}

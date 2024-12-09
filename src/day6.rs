use std::collections::{HashMap, HashSet};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::{fs, thread};

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

#[derive(Clone, Eq, Hash, PartialEq)]
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

#[derive(Clone)]
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

    fn add_obstacle(&mut self, row: u8, col: u8) {
        let obstacle_row = self.rows.entry(row).or_insert_with(Vec::new);
        obstacle_row.push(col);
        obstacle_row.sort();
        let obstacle_col = self.cols.entry(col).or_insert_with(Vec::new);
        obstacle_col.push(row);
        obstacle_col.sort();
    }

    fn remove_obstacle(&mut self, row: u8, col: u8) {
        let obstacle_row = self.rows.get_mut(&row).unwrap();
        let row_i = obstacle_row.iter().position(|&c| c == col).unwrap();
        obstacle_row.remove(row_i);
        let obstacle_col = self.cols.get_mut(&col).unwrap();
        let col_i = obstacle_col.iter().position(|&r| r == row).unwrap();
        obstacle_col.remove(col_i);
    }
}

#[derive(Debug)]
pub struct MapSize {
    pub width: u8,
    pub height: u8,
}

impl MapSize {
    pub fn from_maps(raw_dataset: &str) -> MapSize {
        let width = raw_dataset.lines().next().unwrap().len() as u8;
        let height = raw_dataset.lines().count() as u8;
        MapSize { width, height }
    }
}

struct Path(u8, u8);

struct MovementRecords {
    rows: HashMap<u8, Vec<Path>>,
    cols: HashMap<u8, Vec<Path>>,
    looped: bool,
}

fn get_movement_records(
    map_size: &MapSize,
    obstacles: &ObstacleHashMap,
    guard: &mut GuardPosition,
) -> MovementRecords {
    let mut rows = HashMap::new();
    let mut cols = HashMap::new();
    let mut out_of_map = false;
    let mut turning_points: HashSet<(u8, u8, Direction)> = HashSet::new();
    let mut looped = false;
    while !out_of_map && !looped {
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
        looped = turning_points.contains(&(guard.row, guard.col, guard.facing.clone()));
        turning_points.insert((guard.row, guard.col, guard.facing.clone()));
        guard.turn_right();
    }
    MovementRecords { rows, cols, looped }
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

fn process_first(movement_records: &MovementRecords) -> usize {
    let crossed = count_crossed(movement_records);
    let visited = count_visited(movement_records);
    visited - crossed
}

fn process_second(
    map_size: &MapSize,
    movement_records: &MovementRecords,
    obstacles: &mut ObstacleHashMap,
    guard_original: &GuardPosition,
) -> usize {
    let mut visited: HashSet<(u8, u8)> = HashSet::new();
    for (&row_i, paths_row) in movement_records.rows.iter() {
        for path_row in paths_row {
            let lower_col = path_row.0.min(path_row.1);
            let upper_col = path_row.0.max(path_row.1);
            for col_i in lower_col..=upper_col {
                visited.insert((row_i, col_i));
            }
        }
    }
    for (&col_i, paths_col) in movement_records.cols.iter() {
        for path_col in paths_col {
            let lower_row = path_col.0.min(path_col.1);
            let upper_row = path_col.0.max(path_col.1);
            for row_i in lower_row..=upper_row {
                visited.insert((row_i, col_i));
            }
        }
    }
    let count = Arc::new(AtomicUsize::new(0));
    let thread_count = 8;
    let per_thread = visited.len() / thread_count;
    thread::scope(|scope| {
        let calc = |thread| {
            let count = Arc::clone(&count);
            for (row, col) in (&visited)
                .iter()
                .skip(per_thread as usize * thread as usize)
                .take(per_thread as usize)
                .take(per_thread)
            {
                let mut obstacles = obstacles.clone();
                obstacles.add_obstacle(*row, *col);
                let mut guard = guard_original.clone();
                if get_movement_records(&map_size, &obstacles, &mut guard).looped {
                    count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
                obstacles.remove_obstacle(*row, *col);
            }
        };
        for i in 0..=thread_count {
            scope.spawn(move || calc(i));
        }
    });
    count.load(std::sync::atomic::Ordering::Acquire)
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);
    let map_size = MapSize::from_maps(&raw_dataset);
    let obstacles = ObstacleHashMap::from_maps(&raw_dataset);
    let guard_original = GuardPosition::from_maps(&raw_dataset).unwrap();
    let mut guard = guard_original.clone();
    let mut movement_records = get_movement_records(&map_size, &obstacles, &mut guard);
    simplify_visited(&mut movement_records);

    let distinct_visit = process_first(&movement_records);
    println!("Distinct visit: {}", distinct_visit);

    let mut obstacles = obstacles;
    let can_cause_loop = process_second(
        &map_size,
        &movement_records,
        &mut obstacles,
        &guard_original,
    );
    println!(
        "Positions for new obstacle that can cause loop: {}",
        can_cause_loop
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_ex() {
        let raw_dataset = read_input_file("input/day6_ex.txt");
        let map_size = MapSize::from_maps(&raw_dataset);
        let obstacles = ObstacleHashMap::from_maps(&raw_dataset);
        let guard_original = GuardPosition::from_maps(&raw_dataset).unwrap();
        let mut guard = guard_original.clone();
        let mut movement_records = get_movement_records(&map_size, &obstacles, &mut guard);
        simplify_visited(&mut movement_records);
        let distinct_visit = process_first(&movement_records);
        assert_eq!(distinct_visit, 41);
        let mut obstacles = obstacles;
        let can_cause_loop = process_second(
            &map_size,
            &movement_records,
            &mut obstacles,
            &guard_original,
        );
        assert_eq!(can_cause_loop, 6);
    }

    #[test]
    fn test_process() {
        let raw_dataset = read_input_file("input/day6.txt");
        let map_size = MapSize::from_maps(&raw_dataset);
        let obstacles = ObstacleHashMap::from_maps(&raw_dataset);
        let guard_original = GuardPosition::from_maps(&raw_dataset).unwrap();
        let mut guard = guard_original.clone();
        let mut movement_records = get_movement_records(&map_size, &obstacles, &mut guard);
        simplify_visited(&mut movement_records);
        let distinct_visit = process_first(&movement_records);
        assert_eq!(distinct_visit, 5404);
        let mut obstacles = obstacles;
        let can_cause_loop = process_second(
            &map_size,
            &movement_records,
            &mut obstacles,
            &guard_original,
        );
        assert_eq!(can_cause_loop, 1984);
    }
}

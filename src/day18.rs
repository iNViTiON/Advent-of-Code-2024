use std::{collections::{HashMap, HashSet}, fs, sync::{atomic::{AtomicUsize, Ordering}, Arc}};

use crate::day8::Position;

struct Config {
    in_file: String,
    bytes_limit: usize,
    dimention: u8,
}

impl Config {
    fn new(args: &mut impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let in_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing input file argument"),
        };
        let dimention = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => return Err("Missing map dimention argument"),
        };
        let bytes_limit = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => return Err("Missing bytes limit argument"),
        };

        Ok(Config { in_file, bytes_limit, dimention })
    }
}

fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        std::process::exit(1);
    })
}

fn get_map_limit_byte(raw_dataset: &str, bytes_limit: &usize) -> HashSet<Position> {
    raw_dataset
        .lines()
        .take(*bytes_limit)
        .fold(HashSet::new(), |mut map, line| {
            let mut v = line.split(',')
                .map(|s| s.parse().unwrap());
            map.insert(Position { row: v.next().unwrap(), col: v.next().unwrap() });
            map
        })
}

fn get_possible_moves(
    position: &Position,
    map: &HashSet<Position>,
    map_dimention: &u8,
) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    if position.row > 0 {
        let candidate = Position {
            row: position.row - 1,
            col: position.col,
        };
        if !map.contains(&candidate) {
            moves.push(candidate);
        }
    }
    if position.row < *map_dimention {
        let candidate = Position {
            row: position.row + 1,
            col: position.col,
        };
        if !map.contains(&candidate) {
            moves.push(candidate);
        }
    }
    if position.col > 0 {
        let candidate = Position {
            row: position.row,
            col: position.col - 1,
        };
        if !map.contains(&candidate) {
            moves.push(candidate);
        }
    }
    if position.col < *map_dimention {
        let candidate = Position {
            row: position.row,
            col: position.col + 1,
        };
        if !map.contains(&candidate) {
            moves.push(candidate);
        }
    }
    moves
}

fn next_frontline(
    frontline: &HashSet<Position>,
    dict: &mut HashMap<Position, usize>,
    step: usize,
    map: &HashSet<Position>,
    map_dimention: &u8,
) -> HashSet<Position> {
    frontline.into_iter().fold(HashSet::new(), |mut acc, position| {
        let moves = get_possible_moves(position, map, map_dimention);
        for m in moves {
            if !dict.contains_key(&m) {
                dict.insert(m.clone(), step);
                acc.insert(m);
            }
        }
        acc
    })
}

fn process_first(raw_dataset: &str, bytes_limit: &usize, map_dimention: &u8) -> usize {
    let map = get_map_limit_byte(raw_dataset, bytes_limit);
    let start = Position { row: 0, col: 0 };
    let exit = Position { row: map_dimention - 1, col: map_dimention - 1 };
    let mut frontline_start: HashSet<Position> = HashSet::new();
    let mut frontline_step = 0usize;
    let mut dict_start: HashMap<Position, usize> = HashMap::new();
    let mut frontline_exit: HashSet<Position> = HashSet::new();
    let mut frontline_exit_step = 0usize;
    let mut dict_exit: HashMap<Position, usize> = HashMap::new();
    frontline_start.insert(start.clone());
    dict_start.insert(start.clone(), 0);
    frontline_exit.insert(exit.clone());
    dict_exit.insert(exit.clone(), 0);

    loop {
        if frontline_start.len() == 0 || frontline_exit.len() == 0 {
            break;
        }
        frontline_step += 1;
        frontline_start = next_frontline(
            &frontline_start,
            &mut dict_start,
            frontline_step,
            &map,
            map_dimention,
        );
        if frontline_start.intersection(&frontline_exit).count() > 0 {
            break;
        }
        frontline_exit_step += 1;
        frontline_exit = next_frontline(
            &frontline_exit,
            &mut dict_exit,
            frontline_exit_step,
            &map,
            map_dimention,
        );
        if frontline_start.intersection(&frontline_exit).count() > 0 {
            break;
        }
    }
    frontline_start
        .intersection(&frontline_exit)
        .map(|p| dict_start.get(p).unwrap() + dict_exit.get(p).unwrap())
        .min()
        .unwrap_or_default() as usize
}

fn process_second(raw_dataset: &str, start_at_bytes: &usize, map_dimention: &u8) -> Position {
    // should be much faster if we just find a path (with diagonal move possible) from top right to bottom left
    // and start dropping bytes. The first byte that falls and that path possible is when escape route is cut off
    use rayon::prelude::*;
    let bytes_limit = Arc::new(AtomicUsize::new(usize::MAX));
    (*start_at_bytes..)
        .into_iter()
        .par_bridge()
        .into_par_iter()
        .take_any_while(|_| bytes_limit.load(Ordering::Relaxed) == usize::MAX)
        .for_each(|thread_bytes_limit| {
            if process_first(&raw_dataset, &thread_bytes_limit, map_dimention) == 0 {
                bytes_limit.fetch_min(thread_bytes_limit, Ordering::Relaxed);
            }
        });
    let bytes_limit = bytes_limit.load(Ordering::Acquire);
    let cut_off_position = raw_dataset.lines().skip(bytes_limit - 1).next().unwrap();
    let mut v = cut_off_position.split(',')
        .map(|s| s.parse().unwrap());
    Position { row: v.next().unwrap(), col: v.next().unwrap() }
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);
    let shortest_path_steps = process_first(&raw_dataset, &config.bytes_limit, &config.dimention);
    println!("Shortest path after {} bytes falling required minimum steps: {}", config.bytes_limit, shortest_path_steps);
    let byte_falling_position_cutoff_escape_route = process_second(&raw_dataset, &config.bytes_limit, &config.dimention);
    println!("Byte falling position cutoff escape route at position: {},{}", byte_falling_position_cutoff_escape_route.row, byte_falling_position_cutoff_escape_route.col);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_ex() {
        let raw_dataset = read_input_file("input/day18_ex.txt");
        assert_eq!(process_first(&raw_dataset, &12, &6), 22);
        let byte_falling_position_cutoff_escape_route = process_second(&raw_dataset, &12, &6);
        assert_eq!(byte_falling_position_cutoff_escape_route.row, 6);
        assert_eq!(byte_falling_position_cutoff_escape_route.col, 1);
    }

    #[test]
    fn test_process() {
        let raw_dataset = read_input_file("input/day18.txt");
        assert_eq!(process_first(&raw_dataset, &1024, &70), 318);
        let byte_falling_position_cutoff_escape_route = process_second(&raw_dataset, &1024, &70);
        assert_eq!(byte_falling_position_cutoff_escape_route.row, 56);
        assert_eq!(byte_falling_position_cutoff_escape_route.col, 29);
    }
}

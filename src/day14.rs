use regex_lite::Regex;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU16, Ordering};
use std::{
    fs,
    sync::{atomic::AtomicU8, Arc},
    thread,
};

use crate::{day6::MapSize, day8::Position};

struct Config {
    in_file: String,
    map_size: MapSize,
}

impl Config {
    fn new(args: &mut impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let in_file = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing input file argument"),
        };
        let is_ex = match args.next() {
            Some(_) => true,
            None => false,
        };

        let map_size = match is_ex {
            true => MapSize {
                width: 11,
                height: 7,
            },
            false => MapSize {
                width: 101,
                height: 103,
            },
        };

        Ok(Config { in_file, map_size })
    }
}

fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        std::process::exit(1);
    })
}

struct Robot<'a> {
    initial_position: Position,
    velocity: Position,
    map_size: &'a MapSize,
}

impl<'a> Robot<'a> {
    fn new(
        pos_col: &str,
        pos_row: &str,
        vel_col: &str,
        vel_row: &str,
        map_size: &'a MapSize,
    ) -> Robot<'a> {
        let pos_row: u8 = pos_row.parse().unwrap();
        let pos_col: u8 = pos_col.parse().unwrap();
        let vel_row: i16 = vel_row.parse().unwrap();
        let vel_col: i16 = vel_col.parse().unwrap();
        let vel_row: u8 = if vel_row < 0 {
            (map_size.height as i16 + vel_row) as u8
        } else {
            vel_row as u8
        };
        let vel_col: u8 = if vel_col < 0 {
            (map_size.width as i16 + vel_col) as u8
        } else {
            vel_col as u8
        };
        Robot {
            initial_position: Position {
                row: pos_row,
                col: pos_col,
            },
            velocity: Position {
                row: vel_row,
                col: vel_col,
            },
            map_size,
        }
    }

    fn get_position_at_second(&self, seconds: u16) -> Position {
        let row = (self.initial_position.row as u32 + self.velocity.row as u32 * seconds as u32)
            % self.map_size.height as u32;
        let col = (self.initial_position.col as u32 + self.velocity.col as u32 * seconds as u32)
            % self.map_size.width as u32;
        Position {
            row: row as u8,
            col: col as u8,
        }
    }

    fn get_quadrant_at_second(&self, seconds: &u8) -> Option<u8> {
        let pos = self.get_position_at_second(*seconds as u16);
        let middle_row = self.map_size.height / 2;
        let middle_col = self.map_size.width / 2;
        if pos.row == middle_row || pos.col == middle_col {
            return None;
        }
        let is_left = if pos.col < middle_col { 0u8 } else { 2u8 };
        let is_top = if pos.row < middle_row { 1u8 } else { 2u8 };
        Some(is_left + is_top)
    }
}

fn to_robots<'a>(raw_dataset: &str, map_size: &'a MapSize) -> Vec<Robot<'a>> {
    let regex = Regex::new(r"p=(?<pr>\d+),(?<pc>\d+) v=(?<vr>-?\d+),(?<vc>-?\d+)").unwrap();
    regex
        .captures_iter(raw_dataset)
        .map(|capture| {
            Robot::new(
                capture.name("pr").unwrap().as_str(),
                capture.name("pc").unwrap().as_str(),
                capture.name("vr").unwrap().as_str(),
                capture.name("vc").unwrap().as_str(),
                map_size,
            )
        })
        .collect()
}

fn process_first(robots: &Vec<Robot>, seconds: u8) -> usize {
    let q1 = Arc::new(AtomicU8::new(0));
    let q2 = Arc::new(AtomicU8::new(0));
    let q3 = Arc::new(AtomicU8::new(0));
    let q4 = Arc::new(AtomicU8::new(0));

    let thread_count = thread::available_parallelism().unwrap().get();
    let per_thread = (robots.len() / thread_count) + 1;
    thread::scope(|scope| {
        let calc = |thread_i| {
            let mut counts: [u8; 4] = [0, 0, 0, 0];
            for quadrant in robots
                .iter()
                .skip(thread_i * per_thread)
                .take(per_thread)
                .filter_map(|robot| robot.get_quadrant_at_second(&seconds))
            {
                counts[quadrant as usize - 1] += 1;
            }
            if counts[0] > 0 {
                q1.fetch_add(counts[0], Ordering::Relaxed);
            }
            if counts[1] > 0 {
                q2.fetch_add(counts[1], Ordering::Relaxed);
            }
            if counts[2] > 0 {
                q3.fetch_add(counts[2], Ordering::Relaxed);
            }
            if counts[3] > 0 {
                q4.fetch_add(counts[3], Ordering::Relaxed);
            }
        };
        for i in 0..thread_count {
            scope.spawn(move || calc(i));
        }
    });
    let q1 = q1.load(Ordering::Acquire) as usize;
    let q2 = q2.load(Ordering::Acquire) as usize;
    let q3 = q3.load(Ordering::Acquire) as usize;
    let q4 = q4.load(Ordering::Acquire) as usize;
    q1 * q2 * q3 * q4
}

fn process_second(robots: &Vec<Robot>) -> u16 {
    use rayon::prelude::*;
    let seconds = Arc::new(AtomicU16::new(0));
    (0u16..)
        .into_iter()
        .par_bridge()
        .into_par_iter()
        .take_any_while(|_| seconds.load(Ordering::Relaxed) == 0)
        .for_each(|thread_seconds| {
            let mut positions: HashSet<Position> = HashSet::new();
            for robot in robots.iter() {
                if !positions.insert(robot.get_position_at_second(thread_seconds)) {
                    return;
                }
            }
            let old_seconds = seconds.load(Ordering::Relaxed);
            if old_seconds == 0 || old_seconds > thread_seconds {
                seconds.store(thread_seconds, Ordering::Relaxed);
            }
        });
    seconds.load(Ordering::Acquire) as u16
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);
    let robots = to_robots(&raw_dataset, &config.map_size);
    let safety_factor_at_100_seconds = process_first(&robots, 100);
    println!(
        "Safety factor value at 100 seconds: {}",
        safety_factor_at_100_seconds
    );
    let first_easter_egg_time = process_second(&robots);
    println!(
        "First time Easter egg is display at: {} seconds",
        first_easter_egg_time
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex() {
        let mut args = vec!["input/day14_ex.txt".to_string(), "ex".to_string()].into_iter();
        let config = Config::new(&mut args).unwrap();
        let raw_dataset = read_input_file(&config.in_file);
        let robots = to_robots(&raw_dataset, &config.map_size);
        let safety_factor_at_100_seconds = process_first(&robots, 100);
        assert_eq!(safety_factor_at_100_seconds, 12);
    }

    #[test]
    fn test() {
        let mut args = vec!["input/day14.txt".to_string()].into_iter();
        let config = Config::new(&mut args).unwrap();
        let raw_dataset = read_input_file(&config.in_file);
        let robots = to_robots(&raw_dataset, &config.map_size);
        let safety_factor_at_100_seconds = process_first(&robots, 100);
        assert_eq!(safety_factor_at_100_seconds, 228457125);
        let first_easter_egg_time = process_second(&robots);
        assert_eq!(first_easter_egg_time, 6493);
    }
}

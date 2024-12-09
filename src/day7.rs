use rayon::prelude::*;
use std::sync::{atomic::AtomicU64, Arc};

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

fn read_lines(file_path: &str) -> impl Iterator<Item = String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(file_path).expect("Cannot open file");
    BufReader::new(file).lines().map(|line| line.unwrap())
}

struct InputEquation {
    test_result: u64,
    numbers: Vec<u64>,
}

fn process_first(lines: impl Iterator<Item = String>) -> u64 {
    let sum = Arc::new(AtomicU64::new(0));
    let equations = lines
        .map(|line| {
            let mut line_iter = line.split(":");
            let test_result = line_iter.next().unwrap().parse::<u64>().unwrap();
            let numbers = line_iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            InputEquation {
                test_result,
                numbers,
            }
        })
        .collect::<Vec<InputEquation>>();
    equations.into_par_iter().for_each(|equation| {
        let sum = Arc::clone(&sum);
        let operator_count = equation.numbers.len() - 1;
        let max_operator = 2usize.pow(operator_count as u32);
        for current_operator in 0..max_operator {
            let mut result = equation.numbers[0];
            for i in 0..operator_count {
                if (current_operator >> i) & 1 == 0 {
                    result += equation.numbers[i + 1];
                } else {
                    result *= equation.numbers[i + 1];
                }
            }
            if result == equation.test_result {
                sum.fetch_add(equation.test_result, std::sync::atomic::Ordering::SeqCst);
                break;
            }
        }
    });
    sum.load(std::sync::atomic::Ordering::SeqCst)
}

fn process_second(lines: impl Iterator<Item = String>) -> u64 {
    let sum = Arc::new(AtomicU64::new(0));
    let equations = lines
        .map(|line| {
            let mut line_iter = line.split(":");
            let test_result = line_iter.next().unwrap().parse::<u64>().unwrap();
            let numbers = line_iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            InputEquation {
                test_result,
                numbers,
            }
        })
        .collect::<Vec<InputEquation>>();
    equations.into_par_iter().for_each(|equation| {
        let sum = Arc::clone(&sum);
        let operator_count = equation.numbers.len() - 1;
        let max_operator = 3usize.pow(operator_count as u32);
        for current_operator in 0..max_operator {
            let mut result = equation.numbers[0];
            for i in 0..operator_count {
                match (current_operator / 3usize.pow(i as u32)) % 3 {
                    0 => result += equation.numbers[i + 1],
                    1 => result *= equation.numbers[i + 1],
                    _ => result = format!("{}{}", result, equation.numbers[i + 1]).parse().unwrap(),
                }
            }
            if result == equation.test_result {
                sum.fetch_add(equation.test_result, std::sync::atomic::Ordering::SeqCst);
                break;
            }
        }
    });
    sum.load(std::sync::atomic::Ordering::SeqCst)
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let lines = read_lines(&config.in_file);
    let result = process_first(lines);
    println!("Total calibration result: {}", result);
    let lines = read_lines(&config.in_file);
    let result = process_second(lines);
    println!("Total calibration result with concat: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_first_ex() {
        let lines = read_lines("input/day7_ex.txt");
        let result = process_first(lines);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_process_first() {
        let lines = read_lines("input/day7.txt");
        let result = process_first(lines);
        assert_eq!(result, 3312271365652);
    }

    #[test]
    fn test_process_second_ex() {
        let lines = read_lines("input/day7_ex.txt");
        let result = process_second(lines);
        assert_eq!(result, 11387);
    }

    #[test]
    fn test_process_second() {
        let lines = read_lines("input/day7.txt");
        let result = process_second(lines);
        assert_eq!(result, 509463489296712);
    }
}

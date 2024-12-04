use regex_lite::{Captures, Match, Regex};
use std::fs;

struct Config {
    in_file: String,
}

#[derive(Debug)]
struct Instruction {
    enable: Option<bool>,
    result: Option<usize>,
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

fn to_num(data: Option<Option<Match<'_>>>) -> usize {
    data.unwrap().unwrap().as_str().parse::<usize>().unwrap()
}

fn process_first(raw_dataset: &str) -> usize {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    mul_regex
        .captures_iter(raw_dataset)
        .map(|capture| {
            let mut iter = capture.iter();
            iter.next();
            let left = to_num(iter.next());
            let right = to_num(iter.next());
            left * right
        })
        .sum()
}

fn to_instruction(capture: Captures<'_>) -> Instruction {
    let mut iter = capture.iter();
    let first = iter.next().unwrap().unwrap().as_str();
    match first {
        "do()" => Instruction {
            enable: Some(true),
            result: None,
        },
        "don't()" => Instruction {
            enable: Some(false),
            result: None,
        },
        _ => {
            let left = to_num(iter.next());
            let right = to_num(iter.next());
            Instruction {
                enable: None,
                result: Some(left * right),
            }
        }
    }
}

fn process_second(raw_dataset: &str) -> usize {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enable = true;
    mul_regex
        .captures_iter(raw_dataset)
        .map(|capture| to_instruction(capture))
        .filter(|instruction| match instruction.enable {
            Some(value) => {
                enable = value;
                false
            }
            None => instruction.result.is_some() && enable,
        })
        .map(|instruction| instruction.result.unwrap())
        .sum()
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);
    let raw_dataset = read_input_file(&config.in_file);

    let mul_sum = process_first(&raw_dataset);
    println!("Multiplication sum: {}", mul_sum);

    let enabled_mul_sem = process_second(&raw_dataset);
    println!("Enabled multiplication sum: {}", enabled_mul_sem);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_first_ex() {
        let raw_dataset = read_input_file("input/day3_ex.txt");
        assert_eq!(process_first(&raw_dataset), 161);
    }

    #[test]
    fn test_process_first() {
        let raw_dataset = read_input_file("input/day3.txt");
        assert_eq!(process_first(&raw_dataset), 160672468);
    }

    #[test]
    fn test_process_second_ex() {
        let raw_dataset = read_input_file("input/day3_ex2.txt");
        assert_eq!(process_second(&raw_dataset), 48);
    }

    #[test]
    fn test_process_second() {
        let raw_dataset = read_input_file("input/day3.txt");
        assert_eq!(process_second(&raw_dataset), 84893551);
    }
}

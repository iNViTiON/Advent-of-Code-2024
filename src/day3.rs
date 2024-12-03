use std::fs;
use regex::Regex;

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

fn process_first(raw_dataset: &str) -> usize {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    mul_regex.captures_iter(raw_dataset).map(|capture| {
        let mut iter = capture.iter();
        iter.next();
        let left = iter.next().unwrap().unwrap().as_str().parse::<usize>().unwrap();
        let right = iter.next().unwrap().unwrap().as_str().parse::<usize>().unwrap();
        left * right
    }).sum()
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);
    let raw_dataset = read_input_file(&config.in_file);

    let safe_count = process_first(&raw_dataset);
    println!("Safe report count: {}", safe_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_first() {
        let raw_dataset = read_input_file("input/day3_ex.txt");
        assert_eq!(process_first(&raw_dataset), 161);
    }
}

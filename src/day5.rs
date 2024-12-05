use regex_lite::Regex;
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

#[derive(Debug)]
struct Input<'a> {
    ordering_rules: &'a str,
    updates: &'a str,
}

impl<'a> Input<'a> {
    fn new(raw_dataset: &'a str) -> Input {
        let mut parts = raw_dataset.split("\n\n");
        Input {
            ordering_rules: parts.next().unwrap(),
            updates: parts.next().unwrap(),
        }
    }
}

fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        std::process::exit(1);
    })
}

fn to_ordering_regex_forbidden_rules(ordering_rules: &str) -> Regex {
    let body = ordering_rules
        .lines()
        .map(|line| {
            let mut page_iter = line.split('|');
            let first = page_iter.next().unwrap();
            let latter = page_iter.next().unwrap();
            let rule_str = format!(r"(?:(^|,){},.*?,?{}(,|$))", latter, first);
            rule_str
        })
        .collect::<Vec<String>>()
        .join("|");
    let regex_str = format!(r"(?:{})", body);
    let builder = Regex::new(&regex_str).unwrap();
    builder
}
            

fn process_first(raw_dataset: &str) -> usize {
    let input = Input::new(raw_dataset);
    let forbidden_rules = to_ordering_regex_forbidden_rules(input.ordering_rules);
    let valid_updates = input.updates.lines().filter(|line| forbidden_rules.find(line).is_none());
    let middle_page_sum = valid_updates
        .map(|line| {
            let pages = line.split(',').collect::<Vec<&str>>();
            pages.get(pages.len() / 2).unwrap().parse::<usize>().unwrap()
})
        .sum::<usize>();
    middle_page_sum
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);

    let middle_page_sum = process_first(&raw_dataset);
    println!("Valid update middle page sum: {}", middle_page_sum);

    // let x_mas_count = process_second(&raw_dataset);
    // println!("X-MAS count: {}", x_mas_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_first_ex() {
        let raw_dataset = read_input_file("input/day5_ex.txt");
        assert_eq!(process_first(&raw_dataset), 143);
    }

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
}

use regex_lite::Regex;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::{cmp::Ordering, collections::HashSet, fs};

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

fn to_ordering_regex_forbidden_rules(ordering_rules: &str) -> Vec<Regex> {
    let regex_combined_count = 10;
    let mut ordering_rule_lines = ordering_rules.lines();
    let mut regexes = Vec::new();
    loop {
        let mut processed_pair = Vec::new();
        for _ in 0..regex_combined_count {
            let line = ordering_rule_lines.next();
            if let Some(line) = line {
                let mut page_iter = line.split('|');
                let first = page_iter.next().unwrap();
                let latter = page_iter.next().unwrap();
                let rule_str = format!(r"(?:(^|,){},.*?,?{}(,|$))", latter, first);
                processed_pair.push(rule_str);
            } else {
                break;
            }
        }
        let body = processed_pair.join("|");
        let regex_str = format!(r"(?:{})", body);
        let regex = Regex::new(&regex_str).unwrap();
        regexes.push(regex);
        if processed_pair.len() < regex_combined_count {
            break;
        }
    }
    regexes
}

struct PartOneResult {
    middle_page_sum: usize,
    invalid_updates: Vec<String>,
}

fn process_first(input: &Input) -> PartOneResult {
    let forbidden_rules = to_ordering_regex_forbidden_rules(input.ordering_rules);
    let valid_updates = Arc::new(Mutex::new(Vec::new()));
    let invalid_updates = Arc::new(Mutex::new(Vec::new()));
    let mut handles: VecDeque<JoinHandle<()>> = VecDeque::new();

    let updates: Vec<String> = input
        .updates
        .lines()
        .map(|update| update.to_string())
        .collect::<Vec<String>>();
    for line in updates {
        handles.retain(|handle| !handle.is_finished());
        if handles.len() >= 8 {
            let handle = handles.pop_front();
            if let Some(handle) = handle {
                handle.join().unwrap();
            }
        }
        let forbidden_rules = forbidden_rules.clone();
        let valid_updates = Arc::clone(&valid_updates);
        let invalid_updates = Arc::clone(&invalid_updates);
        let handle = thread::spawn(move || {
            let mut is_valid = true;
            for rule in forbidden_rules.into_iter() {
                if rule.find(&line).is_some() {
                    is_valid = false;
                    break;
                }
            }
            let store = if is_valid { valid_updates } else { invalid_updates };
            let mut store = store.lock().unwrap();
            store.push(line);
        });
        handles.push_back(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let middle_page_sum = valid_updates
        .lock()
        .unwrap()
        .iter()
        .map(|line| {
            let pages = line.split(',').collect::<Vec<&str>>();
            pages
                .get(pages.len() / 2)
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .sum::<usize>();
    let invalid_updates = invalid_updates
        .lock()
        .unwrap()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    PartOneResult {
        middle_page_sum,
        invalid_updates,
    }
}

fn process_second(input: &Input, part_one_result: &PartOneResult) -> usize {
    let ordering_pair = input
        .ordering_rules
        .lines()
        .map(|line| {
            let mut page_iter = line.split('|');
            let first = page_iter.next().unwrap();
            let latter = page_iter.next().unwrap();
            (first, latter)
        })
        .collect::<Vec<(&str, &str)>>();
    let middle_page_sum: usize = part_one_result
        .invalid_updates
        .iter()
        .map(|line| {
            let mut pages = line.split(',').collect::<Vec<&str>>();
            let pages_set = pages.iter().collect::<HashSet<&&str>>();
            let relevant_ordering_pair = ordering_pair
                .iter()
                .filter(|(first, latter)| pages_set.contains(first) && pages_set.contains(latter))
                .copied()
                .collect::<Vec<(&str, &str)>>();
            pages.sort_by(|a, b| {
                let rule = relevant_ordering_pair
                    .iter()
                    .find(|(first, latter)| first == a && latter == b || first == b && latter == a);
                match rule {
                    None => Ordering::Equal,
                    Some((first, _)) => {
                        if first == a {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }
                }
            });
            pages
                .get(pages.len() / 2)
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    middle_page_sum
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);

    println!("WARNING! This is really slow, you can make it 10× faster by running with --release");
    println!("e.g. `cargo run --release 4 input/day4.txt`");
    println!("e.g. `cargo run -r 4 input/day4.txt`");

    let input = Input::new(&raw_dataset);

    let part_one_result = process_first(&input);
    println!(
        "Valid update middle page sum: {}",
        part_one_result.middle_page_sum
    );

    let middle_page_with_fix_sum = process_second(&input, &part_one_result);
    println!(
        "Valid update middle page with fixed sum: {}",
        middle_page_with_fix_sum
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_ex() {
        let raw_dataset = read_input_file("input/day5_ex.txt");
        let input = Input::new(&raw_dataset);
        let part_one_result = process_first(&input);
        assert_eq!(part_one_result.middle_page_sum, 143);
        assert_eq!(process_second(&input, &part_one_result), 123);
    }

    #[test]
    fn test_process() {
        let raw_dataset = read_input_file("input/day5.txt");
        let input = Input::new(&raw_dataset);
        let part_one_result = process_first(&input);
        assert_eq!(part_one_result.middle_page_sum, 6384);
        assert_eq!(process_second(&input, &part_one_result), 5353);
    }
}

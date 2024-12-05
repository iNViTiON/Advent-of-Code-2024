use regex_lite::{RegexBuilder, Regex};
use std::fs;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};
use std::thread;

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

fn get_multiline_regex(distance: &usize) -> String {
    format!("X.{{{}}}M.{{{}}}A.{{{}}}S", distance, distance, distance)
}

fn get_multiline_reverse_regex(distance: &usize) -> String {
    format!("S.{{{}}}A.{{{}}}M.{{{}}}X", distance, distance, distance)
}

fn process_first(raw_dataset: &str) -> usize {
    let line_length = raw_dataset.lines().next().unwrap().len();
    let mut regexs = Vec::new();
    regexs.push(String::from("XMAS"));
    regexs.push(String::from("SAMX"));
    for offset in (line_length - 1)..=(line_length + 1) {
        regexs.push(get_multiline_regex(&offset));
        regexs.push(get_multiline_reverse_regex(&offset));
    }
    let total_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    for regex_str in regexs {
        let total_count = Arc::clone(&total_count);
        let regex_str = regex_str.clone();
        let raw_dataset = raw_dataset.to_string();
        let handle = thread::spawn(move || {
            let mut builder = RegexBuilder::new(&regex_str);
            let regex = builder.dot_matches_new_line(true).build().unwrap();
            let mut count = 0;
            let mut start_offset = 0;
            loop {
                match regex.find_at(&raw_dataset, start_offset) {
                    Some(found) => {
                        count += 1;
                        start_offset = found.start() + 1;
                    }
                    None => break,
                }
            }
            total_count.fetch_add(count, std::sync::atomic::Ordering::Relaxed)
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    total_count.load(std::sync::atomic::Ordering::Acquire)
}

fn process_second(raw_dataset: &str) -> usize {
    let line_length = raw_dataset.lines().next().unwrap().len();
    let mut regexs = Vec::new();
    let distance = line_length - 1;
    regexs.push(format!("M.S(.|\n){{{}}}A(.|\n){{{}}}M.S", distance, distance));
    regexs.push(format!("M.M(.|\n){{{}}}A(.|\n){{{}}}S.S", distance, distance));
    regexs.push(format!("S.M(.|\n){{{}}}A(.|\n){{{}}}S.M", distance, distance));
    regexs.push(format!("S.S(.|\n){{{}}}A(.|\n){{{}}}M.M", distance, distance));
    let total_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    for regex_str in regexs {
        let total_count = Arc::clone(&total_count);
        let regex_str = regex_str.clone();
        let raw_dataset = raw_dataset.to_string();
        let handle = thread::spawn(move || {
            let regex = Regex::new(&regex_str).unwrap();
            let mut count = 0;
            let mut start_offset = 0;
            loop {
                match regex.find_at(&raw_dataset, start_offset) {
                    Some(found) => {
                        count += 1;
                        start_offset = found.start() + 1;
                    }
                    None => break,
                }
            }
            total_count.fetch_add(count, std::sync::atomic::Ordering::Relaxed)
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    total_count.load(std::sync::atomic::Ordering::Acquire)
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
    
    let xmas_count = process_first(&raw_dataset);
    println!("XMAS count: {}", xmas_count);

    let x_mas_count = process_second(&raw_dataset);
    println!("X-MAS count: {}", x_mas_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_first_ex() {
        let raw_dataset = read_input_file("input/day4_ex.txt");
        assert_eq!(process_first(&raw_dataset), 18);
    }

    #[test]
    fn test_process_first() {
        let raw_dataset = read_input_file("input/day4.txt");
        assert_eq!(process_first(&raw_dataset), 2571);
    }

    #[test]
    fn test_process_second_ex() {
        let raw_dataset = read_input_file("input/day4_ex.txt");
        assert_eq!(process_second(&raw_dataset), 9);
    }

    #[test]
    fn test_process_second() {
        let raw_dataset = read_input_file("input/day4.txt");
        assert_eq!(process_second(&raw_dataset), 1992);
    }
}

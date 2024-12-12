use std::{collections::HashMap, fs};

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

fn blink(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    }
    let num_str = stone.to_string();
    if num_str.len() % 2 == 0 {
        let half = num_str.len() / 2;
        let (front, back) = num_str.split_at(half);
        return vec![front.parse().unwrap(), back.parse().unwrap()];
    }
    return vec![stone * 2024];
}

fn process_first(stones: &Vec<usize>) -> usize {
    let mut stones = stones.clone();
    for _ in 0..25 {
        stones = stones.into_iter().flat_map(blink).collect();
    }
    stones.len()
}

fn blink_dfs(stone: &usize, level: u8, dict: &mut HashMap<(usize, u8), usize>) -> usize {
    if level == 0 {
        1
    } else if let Some(&result) = dict.get(&(*stone, level)) {
        result
    } else {
        let result = blink(*stone)
            .into_iter()
            .map(|next_stone| blink_dfs(&next_stone, level - 1, dict))
            .sum();
        dict.insert((*stone, level), result);
        result
    }
}

fn process_second(stones: &Vec<usize>) -> usize {
    let mut dict: HashMap<(usize, u8), usize> = HashMap::new();
    stones
        .into_iter()
        .map(|stone| blink_dfs(&stone, 75, &mut dict))
        .sum()
}

fn to_stones(raw_dataset: &str) -> Vec<usize> {
    raw_dataset
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);

    let stones = to_stones(&raw_dataset);

    let stone_count = process_first(&stones);
    println!("25 blinks {} stones", stone_count);

    let stone_count = process_second(&stones);
    println!("75 blinks {} stones", stone_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_ex() {
        let stones = to_stones(&read_input_file("input/day11_ex.txt"));
        assert_eq!(process_first(&stones), 55312);
        assert_eq!(process_second(&stones), 65601038650482);
    }

    #[test]
    fn test_process() {
        let stones = to_stones(&read_input_file("input/day11.txt"));
        assert_eq!(process_first(&stones), 198089);
        assert_eq!(process_second(&stones), 236302670835517);
    }
}

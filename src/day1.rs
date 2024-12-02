use std::collections::HashMap;
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

fn process_input(file_path: &str) -> (Vec<u32>, Vec<u32>) {
  let input = fs::read_to_string(file_path).unwrap_or_else(|err| {
    eprintln!("Problem reading file: {}", err);
    std::process::exit(1);
  });

  let mut input_left: Vec<u32> = Vec::new();
  let mut input_right: Vec<u32> = Vec::new();

  for line in input.lines() {
    let mut line_iter = line.split_whitespace();
    input_left.push(line_iter.next().unwrap().parse::<u32>().unwrap());
    input_right.push(line_iter.next().unwrap().parse::<u32>().unwrap());
  }

  (input_left, input_right)
}

fn process_first(config: &Config) -> u32 {
  let (mut input_left, mut input_right) = process_input(&config.in_file);
  input_left.sort();
  input_right.sort();

  let left_iter = input_left.iter();
  let right_iter = input_right.iter();

  let mut distances: Vec<u32> = Vec::new();
  for (left, right) in left_iter.zip(right_iter) {
    let distance = (*right as i32) - (*left as i32);
    distances.push(distance.abs() as u32);
  }

  distances.iter().sum::<u32>()
}

fn process_second(config: &Config) -> u32 {
  let (input_left, input_right) = process_input(&config.in_file);
  let mut map: HashMap<&u32, (u8, u8)> = HashMap::new();
  for num in input_left.iter() {
    match map.get_mut(num) {
      Some((count, _)) => *count += 1,
      None => { map.insert(num, (1, 0)); },
    };
  }
  for num in input_right.iter() {
    match map.get_mut(num) {
      Some((_, count)) => *count += 1,
      None => (),
    };
  }
  map.into_iter().map(|(k, (l, r))| {
    k * l as u32 * r as u32
  }).sum()
}

pub fn run(mut args: impl Iterator<Item = String>) {
  let config = Config::new(&mut args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    std::process::exit(1);
  });
  println!("Input file: {}", config.in_file);

  let distance = process_first(&config);
  println!("Distance: {}", distance);

  let similarity = process_second(&config);
  println!("Similarity: {}", similarity);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process_first_ex() {
    let config = Config {
      in_file: "input/day1_ex.txt".to_string(),
    };
    let distance = process_first(&config);
    assert_eq!(distance, 11);
  }

  #[test]
  fn test_process_first() {
    let config = Config {
      in_file: "input/day1.txt".to_string(),
    };
    let distance = process_first(&config);
    assert_eq!(distance, 1258579);
  }
  
  #[test]
  fn test_process_second_ex() {
    let config = Config {
      in_file: "input/day1_ex.txt".to_string(),
    };
    let similarity = process_second(&config);
    assert_eq!(similarity, 31);
  }

  #[test]
  fn test_process_second() {
    let config = Config {
      in_file: "input/day1.txt".to_string(),
    };
    let similarity = process_second(&config);
    assert_eq!(similarity, 23981443);
  }
}

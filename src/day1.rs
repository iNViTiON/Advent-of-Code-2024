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

fn process_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
  let input = fs::read_to_string(file_path).unwrap_or_else(|err| {
    eprintln!("Problem reading file: {}", err);
    std::process::exit(1);
  });

  let mut input_left: Vec<i32> = Vec::new();
  let mut input_right: Vec<i32> = Vec::new();

  for line in input.lines() {
    let mut line_iter = line.split_whitespace();
    input_left.push(line_iter.next().unwrap().parse::<i32>().unwrap());
    input_right.push(line_iter.next().unwrap().parse::<i32>().unwrap());
  }

  (input_left, input_right)
}

fn process_first(config: Config) -> u32 {
  let (mut input_left, mut input_right) = process_input(&config.in_file);
  input_left.sort();
  input_right.sort();

  let left_iter = input_left.iter();
  let mut right_iter = input_right.iter();

  let mut distances: Vec<u32> = Vec::new();
  for left in left_iter {
    let right = right_iter.next().unwrap();
    let distance = right - left;
    distances.push(distance.abs() as u32);
  }

  distances.iter().sum::<u32>()
}

pub fn run(mut args: impl Iterator<Item = String>) {
  let config = Config::new(&mut args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    std::process::exit(1);
  });
  println!("Input file: {}", config.in_file);

  let distance = process_first(config);
  println!("Distance: {}", distance);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process_first() {
    let config = Config {
      in_file: "input/day1_ex.txt".to_string(),
    };
    let distance = process_first(config);
    assert_eq!(distance, 11);
  }
}

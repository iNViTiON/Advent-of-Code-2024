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

pub fn run(mut args: impl Iterator<Item = String>) {
  let config = Config::new(&mut args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    std::process::exit(1);
  });
  println!("Input file: {}", config.in_file);

  let input = fs::read_to_string(config.in_file).unwrap_or_else(|err| {
    eprintln!("Problem reading file: {}", err);
    std::process::exit(1);
  });

  for line in input.lines() {
    println!("{}", line);
  }
}

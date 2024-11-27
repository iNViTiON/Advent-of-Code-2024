pub struct Config {
  pub day: u8,
}

impl Config {
  pub fn new(args: &mut impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();

    let day = match args.next() {
        Some(arg) => match arg.parse::<u8>() {
            Ok(day) => day,
            Err(_) => return Err("Day must be a number"),
        },
        None => return Err("Missing day argument"),
    };

    Ok(Config { day })
  }
}
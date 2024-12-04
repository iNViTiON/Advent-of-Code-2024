mod day1;
mod day2;
mod day3;
mod day4;

use invition_aoc2024::Config;

fn main() {
    let mut args = std::env::args();
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Day: {}", config.day);

    match config.day {
        1 => day1::run(args),
        2 => day2::run(args),
        3 => day3::run(args),
        4 => day4::run(args),
        _ => eprintln!("Day not implemented"),
    }
}

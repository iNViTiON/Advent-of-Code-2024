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

fn read_input_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        std::process::exit(1);
    })
}

struct Machine {
    xa: usize,
    ya: usize,
    xb: usize,
    yb: usize,
    xp: usize,
    yp: usize,
}

fn to_machines(raw_dataset: &str) -> Vec<Machine> {
    let regex = Regex::new(r"Button A: X\+(?<xa>\d+), Y\+(?<ya>\d+)\nButton B: X\+(?<xb>\d+), Y\+(?<yb>\d+)\nPrize: X=(?<xp>\d+), Y=(?<yp>\d+)").unwrap();
    regex
        .captures_iter(raw_dataset)
        .map(|capture| {
            let xa = capture
                .name("xa")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let ya = capture
                .name("ya")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let xb = capture
                .name("xb")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let yb = capture
                .name("yb")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let xp = capture
                .name("xp")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            let yp = capture
                .name("yp")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            Machine {
                xa,
                ya,
                xb,
                yb,
                xp,
                yp,
            }
        })
        .collect()
}

fn solve_min_spent(machine: &Machine, add_to_position: usize) -> Option<usize> {
    let first_part = machine.xa * machine.yb;
    let last_part = machine.xb * machine.ya;
    let swap = first_part < last_part;
    let det = match swap {
        false => first_part - last_part,
        true => last_part - first_part,
    };
    let xp = machine.xp as f64 + add_to_position as f64;
    let yp = machine.yp as f64 + add_to_position as f64;
    if det != 0 {
        let x = match swap {
            false => {
                ((xp * machine.yb as f64) - (machine.xb as f64 * yp))
                    / det as f64
            }
            true => {
                ((machine.xb as f64 * yp) - (xp * machine.yb as f64))
                    / det as f64
            }
        };
        let y = match swap {
            false => {
                ((machine.xa as f64 * yp) - (xp * machine.ya as f64))
                    / det as f64
            }
            true => {
                ((xp * machine.ya as f64) - (machine.xa as f64 * yp))
                    / det as f64
            }
        };
        if x.fract() != 0.0 || y.fract() != 0.0 {
            return None;
        } else {
            Some((x.trunc() as usize * 3) + y.trunc() as usize)
        }
    } else {
        let x_ratio = machine.xa as f64 / machine.xb as f64;
        let y_ratio = machine.ya as f64 / machine.yb as f64;
        let p_ratio = xp / yp;
        if x_ratio == y_ratio && y_ratio == p_ratio {
            if x_ratio < 3f64 {
                Some(machine.xp / machine.xa)
            } else {
                let b = (xp / machine.xb as f64).trunc() as usize;
                let a = (xp - (b * machine.xb) as f64) / machine.xa as f64;
                match a.fract() {
                    0.0 => Some((a.trunc() as usize * 3) + b),
                    _ => None,
                }
            }
        } else {
            None
        }
    }
}

fn process(machines: &Vec<Machine>, prize_offset: usize) -> usize {
    machines
        .iter()
        .filter_map(|machine| solve_min_spent(machine, prize_offset))
        .sum()
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);
    let machines = to_machines(&raw_dataset);

    let spent_first = process(&machines, 0);
    println!("{} token needed", spent_first);

    let spent_second = process(&machines, 10000000000000);
    println!("{} token needed if prize position offset by 10000000000000", spent_second);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_ex() {
        let raw_dataset = read_input_file("input/day13_ex.txt");
        let machines = to_machines(&raw_dataset);
        assert_eq!(process(&machines, 0), 480);
        assert_eq!(process(&machines, 10000000000000), 875318608908);
    }

    #[test]
    fn test_process() {
        let raw_dataset = read_input_file("input/day13.txt");
        let machines = to_machines(&raw_dataset);
        assert_eq!(process(&machines, 0), 37686);
        assert_eq!(process(&machines, 10000000000000), 77204516023437);
    }
}

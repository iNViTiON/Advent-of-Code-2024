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

fn input_to_reports<'a>(input: &'a str) -> impl Iterator<Item = impl Iterator<Item = u8> + 'a> {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|num| num.parse::<u8>().unwrap())
    })
}

fn check_safe(last: u8, level: u8, increasing: Option<bool>) -> bool {
    if last == level {
        return false;
    }
    let is_increasing = match increasing {
        None => level > last,
        Some(is_increasing) => is_increasing,
    };
    let (upper, lower) = if is_increasing {
        (level, last)
    } else {
        (last, level)
    };
    upper >= lower && upper - lower <= 3
}

fn is_report_safe(
    report: impl Iterator<Item = u8>,
    mut last_value: Option<u8>,
    mut increasing: Option<bool>,
) -> (bool, u8) {
    let mut checking_level: u8 = 0;
    for level in report {
        match last_value {
            None => last_value = Some(level),
            Some(last) => {
                if !check_safe(last, level, increasing) {
                    return (false, checking_level);
                }
                last_value = Some(level);
                match increasing {
                    None => increasing = Some(level > last),
                    _ => (),
                }
            }
        }
        checking_level += 1;
    }
    (true, 0)
}

fn process_first(raw_dataset: &str) -> usize {
    input_to_reports(&raw_dataset)
        .map(|report| is_report_safe(report, None, None))
        .filter(|&(safe, _)| safe)
        .count()
}

fn is_report_safe_with_tolerance(report: impl Iterator<Item = u8>) -> bool {
    let report_vec: Vec<u8> = report.collect::<Vec<u8>>();
    let (safe, unsafe_level) = is_report_safe(report_vec.iter().cloned(), None, None);
    if safe {
        return true;
    }
    if unsafe_level > 1 {
        let report_vec_remove_previous = [
            &report_vec[..unsafe_level as usize - 2],
            &report_vec[unsafe_level as usize - 1..],
        ]
        .concat();
        let (safe, _) = is_report_safe(report_vec_remove_previous.iter().cloned(), None, None);
        if safe {
            return true;
        }
    }
    {
        let report_vec_remove_last = [
            &report_vec[..unsafe_level as usize - 1],
            &report_vec[unsafe_level as usize..],
        ]
        .concat();
        let (safe, _) = is_report_safe(report_vec_remove_last.iter().cloned(), None, None);
        if safe {
            return true;
        }
    }
    let report_vec_remove_current = [
        &report_vec[..unsafe_level as usize],
        &report_vec[unsafe_level as usize + 1..],
    ]
    .concat();
    is_report_safe(report_vec_remove_current.iter().cloned(), None, None).0
}

fn process_second(raw_dataset: &str) -> usize {
    input_to_reports(&raw_dataset)
        .map(|report| is_report_safe_with_tolerance(report))
        .filter(|&safe| safe)
        .count()
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);
    let raw_dataset = read_input_file(&config.in_file);

    let safe_count = process_first(&raw_dataset);
    println!("Safe report count: {}", safe_count);

    let safe_with_tolerance_count = process_second(&raw_dataset);
    println!(
        "Safe report with tolerance count: {}",
        safe_with_tolerance_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_first_ex() {
        let config = Config {
            in_file: "input/day2_ex.txt".to_string(),
        };
        let raw_dataset = read_input_file(&config.in_file);
        let safe_count = process_first(&raw_dataset);
        assert_eq!(safe_count, 2);
    }

    #[test]
    fn test_process_first() {
        let config = Config {
            in_file: "input/day2.txt".to_string(),
        };
        let raw_dataset = read_input_file(&config.in_file);
        let safe_count = process_first(&raw_dataset);
        assert_eq!(safe_count, 326);
    }

    #[test]
    fn test_process_second_ex() {
        let config = Config {
            in_file: "input/day2_ex.txt".to_string(),
        };
        let raw_dataset = read_input_file(&config.in_file);
        let safe_with_tolerance_count = process_second(&raw_dataset);
        assert_eq!(safe_with_tolerance_count, 4);
    }

    #[test]
    fn test_process_second() {
        let config = Config {
            in_file: "input/day2.txt".to_string(),
        };
        let raw_dataset = read_input_file(&config.in_file);
        let safe_with_tolerance_count = process_second(&raw_dataset);
        assert_eq!(safe_with_tolerance_count, 381);
    }
}

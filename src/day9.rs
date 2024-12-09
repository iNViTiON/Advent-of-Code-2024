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

fn process_first(file_path: &str) -> usize {
    use std::fs::File;
    use std::io::BufReader;
    use utf8_chars::BufReadCharsExt;

    let file = File::open(file_path).expect("Cannot open file");
    let mut buf_reader = BufReader::new(file);
    let input_stream = buf_reader
        .chars()
        .map(|char| char.unwrap().to_digit(10).unwrap());
    let mut disk: Vec<Option<usize>> = Vec::new();

    for (i, len) in input_stream.enumerate() {
        let is_file = i % 2 == 0;
        let file_id = i / 2;
        for _ in 0..len {
            disk.push(if is_file { Some(file_id) } else { None });
        }
    }

    let mut i = 0;
    while i < disk.len() {
        while disk.get(i).unwrap_or(&Some(0)).is_none() {
            disk.swap_remove(i);
        }
        i += 1;
    }

    disk
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, file_index)| acc + (i * file_index.unwrap()))
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let checksum_compact = process_first(&config.in_file);
    println!("Disk checksum after compacted: {}", checksum_compact);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_first_ex() {
        let input = "input/day9_ex.txt";
        let checksum_compact = process_first(input);
        assert_eq!(checksum_compact, 1928);
    }

    #[test]
    fn test_process_first() {
        let input = "input/day9.txt";
        let checksum_compact = process_first(input);
        assert_eq!(checksum_compact, 6367087064415);
    }
}

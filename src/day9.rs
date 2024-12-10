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

fn get_checksum(disk: Vec<Option<usize>>) -> usize {
    disk.into_iter()
        .enumerate()
        .fold(0, |acc, (i, file_index)| acc + (i * file_index.unwrap()))
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

    get_checksum(disk)
}

#[derive(Clone)]
struct Block {
    id: Option<usize>,
    size: usize,
}

fn process_second(file_path: &str) -> usize {
    use std::fs::File;
    use std::io::BufReader;
    use utf8_chars::BufReadCharsExt;

    let file = File::open(file_path).expect("Cannot open file");
    let mut buf_reader = BufReader::new(file);
    let input_stream = buf_reader
        .chars()
        .map(|char| char.unwrap().to_digit(10).unwrap() as usize);
    let mut disk: Vec<Block> = Vec::new();

    for (i, size) in input_stream.enumerate() {
        let is_file = i % 2 == 0;
        let file_id = i / 2;
        disk.push(Block {
            id: if is_file { Some(file_id) } else { None },
            size,
        });
    }

    let last_file_block = disk.iter().rfind(|block| block.id.is_some()).unwrap();
    let last_file_id = last_file_block.id.unwrap();

    let mut current_defrag_id = last_file_id;
    while current_defrag_id > 0 {
        let file_block_i = disk
            .iter()
            .rposition(|block| {
                if let Some(id) = block.id {
                    id == current_defrag_id
                } else {
                    false
                }
            })
            .unwrap();
        let file_size = disk.get(file_block_i).unwrap().size;

        let free_block_i = disk
            .iter()
            .position(|block| block.id.is_none() && block.size >= file_size);

        if let Some(free_block_i) = free_block_i {
            if free_block_i < file_block_i {
                let free_block = disk.get_mut(free_block_i).unwrap();
                let free_block_size = free_block.size;
                let remaining_size = free_block_size - file_size;

                free_block.id = Some(current_defrag_id);
                free_block.size = file_size;

                let file_block = disk.get_mut(file_block_i).unwrap();
                file_block.id = None;

                if remaining_size > 0 {
                    disk.insert(
                        free_block_i + 1,
                        Block {
                            id: None,
                            size: remaining_size,
                        },
                    );
                }

                let mut i = 0;
                while i < disk.len() - 1 {
                    if disk.get(i).unwrap().id.is_none() && disk.get(i + 1).unwrap().id.is_none() {
                        let next_block_size = disk.get(i + 1).unwrap().size;
                        let block = disk.get_mut(i).unwrap();
                        block.size += next_block_size;
                        disk.remove(i + 1);
                    } else {
                        i += 1;
                    }
                }
            }
        }

        current_defrag_id -= 1;
    }

    let mut i = 0;
    disk.into_iter().fold(0, |mut acc, block| {
        match block.id {
            Some(id) => {
                for _ in 0..block.size {
                    acc += i * id;
                    i += 1;
                }
                acc
            },
            _ => {
                i += block.size;
                acc
            }
        }
    })
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let checksum_compact = process_first(&config.in_file);
    println!("Disk checksum after compacted: {}", checksum_compact);

    let checksum_defrag = process_second(&config.in_file);
    println!("Disk checksum after defragment: {}", checksum_defrag);
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

    #[test]
    fn test_process_second_ex() {
        let input = "input/day9_ex.txt";
        let checksum_defrag = process_second(input);
        assert_eq!(checksum_defrag, 2858);
    }

    #[test]
    fn test_process_second() {
        let input = "input/day9.txt";
        let checksum_defrag = process_second(input);
        assert_eq!(checksum_defrag, 6390781891880);
    }
}

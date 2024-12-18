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

enum OperandError {
    ComboOutOfRange,
}

#[derive(Debug)]
enum Operand {
    Literal(u64),
    Combo(u8),
}

impl Operand {
    fn literal_from(operand: u64) -> Result<Operand, OperandError> {
        Ok(Operand::Literal(operand))
    }

    fn combo_from(operand: u8) -> Result<Operand, OperandError> {
        if operand > 7 {
            Err(OperandError::ComboOutOfRange)
        } else {
            Ok(Operand::Combo(operand))
        }
    }
}

enum Instruction {
    ADV(Operand),
    BXL(Operand),
    BST(Operand),
    JNZ(Operand),
    BXC,
    OUT(Operand),
    BDV(Operand),
    CDV(Operand),
}

struct ChronospatialComputer {
    registers: [u64; 3],
    instruction_pointer: usize,
    instructions: Vec<Instruction>,
    output: Vec<u64>,
}

impl ChronospatialComputer {
    fn new(raw_dataset: &str) -> ChronospatialComputer {
        let mut registers = [0; 3];
        for (i, v) in raw_dataset.lines().enumerate().take(3) {
            registers[i] = v[12..].parse().unwrap();
        }
        let raw_instructions = &raw_dataset.lines().skip(4).next().unwrap()[9..];
        let instructions = ChronospatialComputer::to_instructions(raw_instructions);
        ChronospatialComputer {
            registers,
            instruction_pointer: 0,
            instructions,
            output: Vec::new(),
        }
    }

    fn to_instructions(raw_instructions: &str) -> Vec<Instruction> {
        let opcode_combo: [u8; 5] = [0, 2, 5, 6, 7];
        let mut raw = raw_instructions.split(',');
        let mut instructions = Vec::new();
        loop {
            let opcode = raw.next();
            let operand = raw.next();
            if operand.is_none() {
                break;
            }
            let opcode: u8 = opcode.unwrap().parse().unwrap();
            let operand: u64 = operand.unwrap().parse().unwrap();
            let operand = if opcode_combo.contains(&opcode) {
                Operand::combo_from(operand as u8)
            } else {
                Operand::literal_from(operand)
            };
            let instruction = match operand {
                Err(OperandError::ComboOutOfRange) => panic!("Invalid operand"),
                Ok(operand) => match opcode {
                    0 => Instruction::ADV(operand),
                    1 => Instruction::BXL(operand),
                    2 => Instruction::BST(operand),
                    3 => Instruction::JNZ(operand),
                    4 => Instruction::BXC,
                    5 => Instruction::OUT(operand),
                    6 => Instruction::BDV(operand),
                    7 => Instruction::CDV(operand),
                    _ => panic!("Invalid opcode"),
                },
            };
            instructions.push(instruction);
        }
        instructions
    }

    fn get_operant_value(&self, operand: &Operand) -> u64 {
        match operand {
            Operand::Literal(v) => *v,
            Operand::Combo(v) => match v {
                v if *v <= 3 => *v as u64,
                4 => self.registers[0],
                5 => self.registers[1],
                6 => self.registers[2],
                _ => panic!("Should not happen"),
            },
        }
    }

    fn find_initial_register_a_from_output(output: &str) -> u64 {
        let output_iter = output.split(',').map(|v| v.parse::<u64>().unwrap());
        let k1 = output_iter.clone().skip(3).next().unwrap();
        let k2 = output_iter.clone().skip(7).next().unwrap();
        let get_out_b = |a: u64| -> u64 {
            let b = a & 7 ^ k1;
            let c = a >> b;
            b ^ k2 ^ c & 7
        };

        output_iter
            .rev()
            .fold(vec![0], |acc, v| {
                acc.into_iter()
                    .map(|a| a << 3)
                    .flat_map(|a| (0..=7).map(move |frag| a | frag))
                    .filter(|a| get_out_b(*a) == v)
                    .collect()
            })
            .into_iter()
            .min()
            .unwrap()
    }

    fn run(&mut self) {
        loop {
            let instruction = self.instructions.get(self.instruction_pointer);
            if instruction.is_none() {
                break;
            }
            let instruction = instruction.unwrap();
            match instruction {
                Instruction::ADV(operand) => {
                    self.registers[0] >>= self.get_operant_value(operand);
                }
                Instruction::BXL(operand) => {
                    self.registers[1] ^= self.get_operant_value(operand);
                }
                Instruction::BST(operand) => {
                    self.registers[1] = self.get_operant_value(operand) & 0b111;
                }
                Instruction::JNZ(operand) => {
                    if self.registers[0] != 0 {
                        self.instruction_pointer = self.get_operant_value(operand) as usize;
                        continue;
                    }
                }
                Instruction::BXC => {
                    self.registers[1] ^= self.registers[2];
                }
                Instruction::OUT(operand) => {
                    self.output.push(self.get_operant_value(operand) & 0b111);
                }
                Instruction::BDV(operand) => {
                    self.registers[1] = self.registers[0] >> self.get_operant_value(operand);
                }
                Instruction::CDV(operand) => {
                    self.registers[2] = self.registers[0] >> self.get_operant_value(operand);
                }
            }
            self.instruction_pointer += 1;
        }
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

pub fn run(mut args: impl Iterator<Item = String>) {
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Input file: {}", config.in_file);

    let raw_dataset = read_input_file(&config.in_file);
    let mut computer = ChronospatialComputer::new(&raw_dataset);
    computer.run();
    let first_output = computer.get_output();
    println!("Program output: {first_output}");

    let original_instructions = raw_dataset.lines().skip(4).next().unwrap()[9..].to_string();
    let init_register_a =
        ChronospatialComputer::find_initial_register_a_from_output(&original_instructions);
    println!("register A cause output as input: {}", init_register_a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_first_part() {
        let mut args = vec!["input/day17_ex.txt".to_string()].into_iter();
        let config = Config::new(&mut args).unwrap();
        let raw_dataset = read_input_file(&config.in_file);
        let mut computer = ChronospatialComputer::new(&raw_dataset);
        computer.run();
        assert_eq!(computer.get_output(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_process() {
        let mut args = vec!["input/day17.txt".to_string()].into_iter();
        let config = Config::new(&mut args).unwrap();
        let raw_dataset = read_input_file(&config.in_file);
        let mut computer = ChronospatialComputer::new(&raw_dataset);
        computer.run();
        assert_eq!(computer.get_output(), "1,2,3,1,3,2,5,3,1");
        let init_register_a = ChronospatialComputer::find_initial_register_a_from_output(
            "2,4,1,5,7,5,1,6,0,3,4,3,5,5,3,0",
        );
        assert_eq!(init_register_a, 105706277661082);
    }
}

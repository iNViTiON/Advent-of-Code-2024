use std::{fs, num::ParseIntError};

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
    ParseError(ParseIntError),
    ComboOutOfRange,
}

#[derive(Debug)]
enum Operand {
    Literal(u32),
    Combo(u8),
}

impl Operand {
    fn literal_from_str(operand_str: &str) -> Result<Operand, OperandError> {
        operand_str
            .parse()
            .map(Operand::Literal)
            .map_err(OperandError::ParseError)
    }

    fn combo_from_str(operand_str: &str) -> Result<Operand, OperandError> {
        let number = operand_str.parse().map_err(OperandError::ParseError)?;
        if number > 7 {
            Err(OperandError::ComboOutOfRange)
        } else {
            Ok(Operand::Combo(number))
        }
    }

    fn literal_from(operand: u32) -> Result<Operand, OperandError> {
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
    registers: [u32; 3],
    instruction_pointer: usize,
    instructions: Vec<Instruction>,
    output: Vec<u32>,
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
            let operand: u32 = operand.unwrap().parse().unwrap();
            let operand = if opcode_combo.contains(&opcode) {
                Operand::combo_from(operand as u8)
            } else {
                Operand::literal_from(operand)
            };
            let instruction = match operand {
                Err(OperandError::ParseError(_)) => panic!("Invalid operand"),
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

    fn get_operant_value(&self, operand: &Operand) -> u32 {
        match operand {
            Operand::Literal(v) => *v,
            Operand::Combo(v) => match v {
                v if *v <= 3 => *v as u32,
                4 => self.registers[0],
                5 => self.registers[1],
                6 => self.registers[2],
                _ => panic!("Should not happen"),
            },
        }
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
                    println!(
                        "ADV with {:?} (v:{} - {:b})",
                        operand,
                        self.get_operant_value(operand),
                        self.get_operant_value(operand)
                    );
                    self.registers[0] /= 2u32.pow(self.get_operant_value(operand));
                }
                Instruction::BXL(operand) => {
                    println!(
                        "BXL with {:?} (v:{} - {:b})",
                        operand,
                        self.get_operant_value(operand),
                        self.get_operant_value(operand)
                    );
                    self.registers[1] ^= self.get_operant_value(operand);
                }
                Instruction::BST(operand) => {
                    println!(
                        "BST with {:?} (v:{} - {:b})",
                        operand,
                        self.get_operant_value(operand),
                        self.get_operant_value(operand)
                    );
                    self.registers[1] = self.get_operant_value(operand) & 0b111;
                }
                Instruction::JNZ(operand) => {
                    println!(
                        "JNZ with {:?} (v:{} - {:b})",
                        operand,
                        self.get_operant_value(operand),
                        self.get_operant_value(operand)
                    );
                    if self.registers[0] != 0 {
                        println!(
                            "Jumping from {} to {}",
                            self.instruction_pointer,
                            self.get_operant_value(operand)
                        );
                        println!(
                            "{:?} {:b} {:b} {:b}",
                            self.registers, self.registers[0], self.registers[1], self.registers[2]
                        );
                        self.instruction_pointer = self.get_operant_value(operand) as usize;
                        continue;
                    }
                }
                Instruction::BXC => {
                    println!("BXC");
                    self.registers[1] ^= self.registers[2];
                }
                Instruction::OUT(operand) => {
                    println!(
                        "OUT with {:?} (v:{} - {:b})",
                        operand,
                        self.get_operant_value(operand),
                        self.get_operant_value(operand)
                    );
                    self.output.push(self.get_operant_value(operand) & 0b111);
                }
                Instruction::BDV(operand) => {
                    println!(
                        "BDV with {:?} (v:{} - {:b})",
                        operand,
                        self.get_operant_value(operand),
                        self.get_operant_value(operand)
                    );
                    self.registers[1] =
                        self.registers[0] / 2u32.pow(self.get_operant_value(operand));
                }
                Instruction::CDV(operand) => {
                    println!(
                        "CDV with {:?} (v:{} - {:b})",
                        operand,
                        self.get_operant_value(operand),
                        self.get_operant_value(operand)
                    );
                    self.registers[2] =
                        self.registers[0] / 2u32.pow(self.get_operant_value(operand));
                }
            }
            self.instruction_pointer += 1;
            println!(
                "{:?} {:b} {:b} {:b}",
                self.registers, self.registers[0], self.registers[1], self.registers[2]
            );
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
    println!("First part output: {first_output}");
}

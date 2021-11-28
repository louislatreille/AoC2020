use std::collections::HashSet;
use std::fmt::Display;
use std::io::{prelude::*, BufReader};
use std::fs::File;
use lazy_static::lazy_static;
use regex::Regex;

pub fn entry() {
	println!("Starting challenges for day eight!");

	let instruction_strings = read_instructions("./resources/day_eight_input.txt");

    let mut instructions = vec!();
    for instruction_string in instruction_strings {
        let instr = Instruction::new(&instruction_string);
        println!("Found instruction {}", instr);
        instructions.push(instr);
    }

    let (success, acc) = execute_program(&instructions);

    if success {
        println!("Program executed successfully! Acc is at {}", acc);
    } else {
        println!("Tried to execute an instruction a second time! Acc is at {}", acc);
    }
    
    let mut index = 0;
    loop {
        let init_instr = instructions.remove(index);
        let new_instr = init_instr.swap();
        instructions.insert(index, new_instr);

        let (success, acc) = execute_program(&instructions);

        if success {
            println!("Program executed successfully! Acc is at {}", acc);
            break;
        }

        instructions.remove(index);
        instructions.insert(index, init_instr);

        index += 1;
    }    
}

lazy_static! {
    static ref RE_NOP: Regex = Regex::new(r"nop").unwrap();
    static ref RE_ACC: Regex = Regex::new(r"acc ([\+-]\d+)").unwrap();
    static ref RE_JMP: Regex = Regex::new(r"jmp ([\+-]\d+)").unwrap();
}

fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

fn read_instructions(filename: &str) -> Vec<String> {
	let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

	let mut boarding_passes = vec!();
    for line in reader.lines() {
        let line = line.unwrap();
        boarding_passes.push(line);
    }

    boarding_passes
}

fn execute_program(instructions: &Vec<Instruction>) -> (bool, i32) {
    let mut acc = 0;
    let mut executed_indexes: HashSet<usize> = HashSet::new();
    let mut next_index: usize = 0;

    while !executed_indexes.contains(&next_index) {
        println!("Now at index {}", next_index);
        let instruction = match instructions.get(next_index) {
            Some(val) => val,
            None => {
                if next_index == instructions.len() {
                    return (true, acc);
                } else {
                    panic!("Received an instruction that would overflow!");
                }
            },
        };

        println!("Executing instruction {}", instruction);
        executed_indexes.insert(next_index);

        acc += instruction.acc_inc();
        
        next_index = match add(next_index, instruction.next_instr_index()) {
            Some(val) => {
                val
            },
            None => panic!("Received an instruction that would overflow!"),
        };
    }
    
    return (false, acc);
}

struct Instruction {
    code: InstructionCode,
    value: i32
}

#[derive(Debug)]
enum InstructionCode {
    NOP,
    ACC,
    JMP,
}

impl Clone for InstructionCode {
    fn clone(&self) -> Self {
        match self {
            Self::NOP => Self::NOP,
            Self::ACC => Self::ACC,
            Self::JMP => Self::JMP,
        }
    }
}

impl Instruction {
    fn new(instr_str: &str) -> Instruction {
        let caps = RE_NOP.captures(instr_str);
        if caps.is_some() {
            return Instruction {
                code: InstructionCode::NOP,
                value: 0
            }
        }

        let caps = RE_ACC.captures(instr_str);
        if caps.is_some() {
            return Instruction {
                code: InstructionCode::ACC,
                value: caps.unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap()
            }
        }

        let caps = RE_JMP.captures(instr_str);
        if caps.is_some() {
            return Instruction {
                code: InstructionCode::JMP,
                value: caps.unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap()
            }
        } else {
            panic!("Unknown instruction!");
        }
    }

    fn next_instr_index(&self) -> i32 {
        match &self.code {
            InstructionCode::NOP => 1,
            InstructionCode::ACC => 1,
            InstructionCode::JMP => self.value
        }
    }

    fn acc_inc(&self) -> i32 {
        match &self.code {
            InstructionCode::NOP => 0,
            InstructionCode::ACC => self.value,
            InstructionCode::JMP => 0
        }
    }

    fn swap(&self) -> Instruction {
        Instruction {
            code: match &self.code {
                InstructionCode::NOP => InstructionCode::JMP,
                InstructionCode::ACC => InstructionCode::ACC,
                InstructionCode::JMP => InstructionCode::NOP,
            },
            value: self.value
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.code, self.value)
    }
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        Self { code: self.code.clone(), value: self.value.clone() }
    }
}
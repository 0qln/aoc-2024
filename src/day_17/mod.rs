use std::{collections::LinkedList, num::ParseIntError, str::FromStr};

use itertools::Itertools;
use regex::Regex;

pub mod part_1;
pub mod part_2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Byte(u64);

impl Byte {
    const MASK: u64 = 0b111;

    pub fn new(v: u64) -> Byte {
        Byte(v & Self::MASK)
    }
}

impl FromStr for Byte {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Byte(s.parse()?))
    }
}

impl ToString for Byte {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<Byte> for OpCode {
    fn from(v: Byte) -> Self {
        match v.0 {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => unreachable!(),
        }
    }
}

pub type Operand = Byte;

#[derive(Debug)]
pub struct Program(Vec<Byte>);

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len() && self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
    }
}

impl Program {
    pub fn get(&self, index: usize) -> Option<Byte> {
        self.0.get(index).copied()
    }

    pub fn get_opcode(&self, index: usize) -> Option<OpCode> {
        self.get(index).map(|v| OpCode::from(v))
    }

    pub fn get_operand(&self, index: usize) -> Option<Operand> {
        self.get(index)
    }
}

pub type Register = u64;

#[derive(Debug, Default, Clone)]
pub struct Device {
    register_a: Register,
    register_b: Register,
    register_c: Register,
    instruction_ptr: usize,
}

impl Device {
    pub fn eval_combo(&self, operand: Operand) -> u64 {
        match operand {
            byte => match byte.0 {
                0..=3 => byte.0,
                4 => self.register_a,
                5 => self.register_b,
                6 => self.register_c,
                7 => panic!("Combo operand 7 is reserved and will not appear in valid programs."),
                _ => unreachable!(),
            },
        }
    }
    
    fn execute(&self, program: &Program) -> Vec<Byte> {
        ProgramExecution(self.clone(), program).collect()
    }
}

struct ProgramExecution<'a>(Device, &'a Program);

impl Iterator for ProgramExecution<'_> {
    type Item = Byte;

    fn next(&mut self) -> Option<Self::Item> {
        let op_code = self.1.get_opcode(self.0.instruction_ptr)?;
        let operand = self.1.get_operand(self.0.instruction_ptr + 1)?;
        match op_code {
            OpCode::Adv => {
                let num = self.0.register_a;
                let den = 1 << self.0.eval_combo(operand);
                let quo = num / den;
                self.0.register_a = quo;
                self.0.instruction_ptr += 2;
                self.next()
            }
            OpCode::Bxl => {
                self.0.register_b ^= operand.0;
                self.0.instruction_ptr += 2;
                self.next()
            }
            OpCode::Bst => {
                self.0.register_b = Byte::new(self.0.eval_combo(operand)).0;
                self.0.instruction_ptr += 2;
                self.next()
            }
            OpCode::Jnz => {
                if self.0.register_a != 0 {
                    self.0.instruction_ptr = operand.0 as usize;
                } else {
                    self.0.instruction_ptr += 2;
                }
                self.next()
            }
            OpCode::Bxc => {
                self.0.register_b ^= self.0.register_c;
                self.0.instruction_ptr += 2;
                self.next()
            }
            OpCode::Out => {
                self.0.instruction_ptr += 2;
                Some(Byte::new(self.0.eval_combo(operand)))
            }
            OpCode::Bdv => {
                let num = self.0.register_a;
                let den = 1 << self.0.eval_combo(operand);
                let quo = num / den;
                self.0.register_b = quo;
                self.0.instruction_ptr += 2;
                self.next()
            }
            OpCode::Cdv => {
                let num = self.0.register_a;
                let den = 1 << self.0.eval_combo(operand);
                let quo = num / den;
                self.0.register_c = quo;
                self.0.instruction_ptr += 2;
                self.next()
            }
        }
    }
}

pub fn parse(input: &str) -> (Device, Program) {
    let device = Device {
        register_a: Regex::new(r"A: (\d+)").unwrap().captures(input).unwrap()[1]
            .parse()
            .unwrap(),
        register_b: Regex::new(r"B: (\d+)").unwrap().captures(input).unwrap()[1]
            .parse()
            .unwrap(),
        register_c: Regex::new(r"C: (\d+)").unwrap().captures(input).unwrap()[1]
            .parse()
            .unwrap(),
        instruction_ptr: 0,
    };
    let program = Program(
        Regex::new(r"Program: ((\d,?)+)")
            .unwrap()
            .captures(input)
            .unwrap()[1]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect_vec(),
    );
    (device, program)
}

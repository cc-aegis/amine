use std::str::FromStr;
use crate::AssemblerError;
use crate::opcode::{NoOpOpcode, SingleOpOpcode, TwoOpOpcode};
use crate::operand::Operand;

pub enum Instruction {
    Label(String),
    Sublabel(String),
    Include(String),
    Define(String, String),
    RawWords(Vec<Operand>),
    TwoOp(TwoOpOpcode, Operand, Operand),
    SingleOp(SingleOpOpcode, Operand),
    NoOp(NoOpOpcode),
    Blank,
}

pub struct InvalidInstruction;

impl FromStr for Instruction {
    type Err = AssemblerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(';').next().unwrap().trim();
        //todo: match this (pattern if cond => val)
        if let Some(label) = s.strip_suffix(':') {
            return if let Some(label) = s.strip_prefix('.') {
                Ok(Instruction::Sublabel(label[1..].to_string()))
            } else {
                Ok(Instruction::Label(label.to_string()))
            }
        }
        let parts = s.split_whitespace().collect::<Vec<_>>();
        match parts.as_slice() {
            ["include", file] => Ok(Instruction::Include(file.to_string())),
            ["define", lhs, rhs] => Ok(Instruction::Define(lhs.to_string(), rhs.to_string())),
            ["dw", ..] => Ok(Instruction::RawWords(parts
                .into_iter()
                .skip(1)
                .map(str::parse)
                .collect::<Result<_, _>>()
                .map_err(AssemblerError::InvalidOperand)?
            )),
            [opcode, lhs, rhs] => Ok(Instruction::TwoOp(match *opcode {
                "mov" => TwoOpOpcode::Mov,
                "pusht" => TwoOpOpcode::PushT,
                "popt" => TwoOpOpcode::PopT,
                _ => todo!(),
            }, lhs.parse().map_err(AssemblerError::InvalidOperand)?, rhs.parse().map_err(AssemblerError::InvalidOperand)?)),
            [opcode, op] => Ok(Instruction::SingleOp(match *opcode {
                _ => todo!(),
            }, op.parse().map_err(AssemblerError::InvalidOperand)?)),
            [opcode] => Ok(Instruction::NoOp(match *opcode {
                _ => todo!(),
            })),
            [] => Ok(Instruction::Blank),
            _ => todo!(),
        }
    }
}

impl Instruction {
    fn compile_into(&self, /*something to compile operands (HashMap<(String, String) (?), u16>),*/ target: &mut Vec<u16>) {
        match self {
            _ => todo!()
        }
    }
}




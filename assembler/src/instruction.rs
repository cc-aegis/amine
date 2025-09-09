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

pub struct InvalidInstruction(String);

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
                "write" => TwoOpOpcode::Write,
                "copy" => TwoOpOpcode::Copy,
                "swap" => TwoOpOpcode::Swap,
                "readitr" => TwoOpOpcode::ReadItr,
                "writeitr" => TwoOpOpcode::WriteItr,
                "copyitr" => TwoOpOpcode::CopyItr,
                "lookup" => TwoOpOpcode::Lookup,
                "jlookup" => TwoOpOpcode::JLookup,
                "clookup" => TwoOpOpcode::CLookup,
                "jrnzdec" => TwoOpOpcode::JrnzDec,
                "callw" => TwoOpOpcode::CallW,
                "jrz" => TwoOpOpcode::Jrz,
                "jrnz" => TwoOpOpcode::Jrnz,
                "jrgt" => TwoOpOpcode::Jrgt,
                "jrge" => TwoOpOpcode::Jrge,
                "jrlt" => TwoOpOpcode::Jrlt,
                "jrle" => TwoOpOpcode::Jrle,
                "add" => TwoOpOpcode::Add,
                "sub" => TwoOpOpcode::Sub,
                "mul" => TwoOpOpcode::Mul,
                "div" => TwoOpOpcode::Div,
                "utof" => TwoOpOpcode::Utof,
                "itof" => TwoOpOpcode::Itof,
                "imul" => TwoOpOpcode::Imul,
                "idiv" => TwoOpOpcode::Idiv,
                "fadd" => TwoOpOpcode::Fadd,
                "fsub" => TwoOpOpcode::Fsub,
                "fmul" => TwoOpOpcode::Fmul,
                "fdiv" => TwoOpOpcode::Fdiv,
                "ftou" => TwoOpOpcode::Ftou,
                "ftoi" => TwoOpOpcode::Ftoi,
                "and" => TwoOpOpcode::And,
                "or" => TwoOpOpcode::Or,
                "xor" => TwoOpOpcode::Xor,
                "inv" => TwoOpOpcode::Inv,
                "bool" => TwoOpOpcode::Bool,
                "neg" => TwoOpOpcode::Neg,
                "shl" => TwoOpOpcode::Shl,
                "shr" => TwoOpOpcode::Shr,
                "cmp" => TwoOpOpcode::Cmp,
                "ctx" => TwoOpOpcode::Ctx,
                _ => Err(AssemblerError::InvalidInstruction(InvalidInstruction(s.to_string())))?,
            }, lhs.parse().map_err(AssemblerError::InvalidOperand)?, rhs.parse().map_err(AssemblerError::InvalidOperand)?)),
            [opcode, op] => Ok(Instruction::SingleOp(match *opcode {
                "dbg" => SingleOpOpcode::Dbg,
                "push" => SingleOpOpcode::Push,
                "pop" => SingleOpOpcode::Pop,
                "cmpz" => SingleOpOpcode::CmpZ,
                "inc" => SingleOpOpcode::Inc,
                "dec" => SingleOpOpcode::Dec,
                "sshl" => SingleOpOpcode::SShl,
                "sshr" => SingleOpOpcode::SShr,
                "floor" => SingleOpOpcode::Floor,
                "ceil" => SingleOpOpcode::Ceil,
                "jz" => SingleOpOpcode::Jz,
                "jnz" => SingleOpOpcode::Jnz,
                "jgt" => SingleOpOpcode::Jgt,
                "jge" => SingleOpOpcode::Jge,
                "jlt" => SingleOpOpcode::Jlt,
                "jle" => SingleOpOpcode::Jle,
                "jmp" => SingleOpOpcode::Jmp,
                "call" => SingleOpOpcode::Call,
                "retv" => SingleOpOpcode::RetV,
                _ => Err(AssemblerError::InvalidInstruction(InvalidInstruction(s.to_string())))?,
            }, op.parse().map_err(AssemblerError::InvalidOperand)?)),
            [opcode] => Ok(Instruction::NoOp(match *opcode {
                "nop" => NoOpOpcode::Nop,
                "ret" => NoOpOpcode::Ret,
                "send" => NoOpOpcode::Send,
                "exit" => NoOpOpcode::Exit,
                _ => Err(AssemblerError::InvalidInstruction(InvalidInstruction(s.to_string())))?,
            })),
            [] => Ok(Instruction::Blank),
            _ => Err(AssemblerError::InvalidInstruction(InvalidInstruction(s.to_string())))?,
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




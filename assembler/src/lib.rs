#![feature(f16)]

use std::path::Path;
use crate::instruction::Instruction;

mod instruction;
mod opcode;
mod operand;

pub enum AssemblerError {
    FileNotFound(String),
    InvalidInstruction(instruction::InvalidInstruction),
    InvalidOperand(operand::InvalidOperand),
}
fn compile_file(mut root_dir: &Path, file: &str) -> Result<Vec<Instruction>, AssemblerError> {
    std::fs::read_to_string(&root_dir.join(file))
        .map_err(|_| file.to_string())
        .map_err(AssemblerError::FileNotFound)?
        .lines()
        .map(str::parse)
        .collect()
}

pub fn link(root_dir: &Path) -> Result<Vec<Instruction>, AssemblerError> {
    compile_file(root_dir, "main.s")
}

pub fn assemble(code: &[Instruction]) -> Box<[u16; 65536]> {
    todo!()
}
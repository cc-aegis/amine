#![feature(f16)]

use std::collections::HashSet;
use std::path::Path;
use crate::instruction::Instruction;
use crate::preparse::{preparse};

mod instruction;
mod opcode;
mod operand;
mod preparse;
mod link;
mod assemble;

// TODO: move all error variants into here
#[derive(Debug)]
pub enum AssemblerError {
    FileNotFound(String),
    InvalidInstruction(instruction::InvalidInstruction),
    InvalidOperand(operand::InvalidOperand),
    UnallowedRegister(operand::UnallowedRegister),
    InvalidLabel(operand::InvalidLabel),
    UnallowedInclude(preparse::UnallowedInclude),
    CodeTooLong,
}

fn parse_file(mut root_dir: &Path, file: &str) -> Result<Vec<Instruction>, AssemblerError> {
    std::fs::read_to_string(&root_dir.join(file))
        .map_err(|_| file.to_string())
        .map_err(AssemblerError::FileNotFound)?
        .lines()
        .map(str::parse)
        .collect()
}

fn link_rec(root_dir: &Path, file: String, blacklist: &mut HashSet<String>) -> Result<Vec<Instruction>, AssemblerError> {
    if blacklist.contains(&file) {
        return Ok(Vec::new());
    }

    let parsed = parse_file(root_dir, &file)?;

    blacklist.insert(file);

    let result = parsed
        .into_iter()
        .map(|inst| match inst {
            Instruction::Include(file) => link_rec(root_dir, file, blacklist),
            _ => Ok(vec![inst]),
        })
        .collect::<Result<Vec<Vec<Instruction>>, AssemblerError>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<Instruction>>();

    Ok(result)
}

pub fn link(root_dir: &Path) -> Result<Vec<Instruction>, AssemblerError> {
    // parse_file(root_dir, "main.s")
    link_rec(root_dir, String::from("main.s"), &mut HashSet::new())
}

pub fn assemble(code: &[Instruction]) -> Result<Box<[u16; 65536]>, AssemblerError> {
    let code = assemble::assemble(code)?;
    let mut result = Box::new([0; 65536]);

    match code.len() {
        65537.. => Err(AssemblerError::CodeTooLong),
        len => {
            result[..len].copy_from_slice(&code);
            Ok(result)
        }
    }
}
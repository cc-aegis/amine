use crate::AssemblerError;
use crate::instruction::Instruction;
use crate::preparse::preparse;

struct CircularDefinition;
struct NonexistentDefinition;

// TODO: idea: Iter<Instruction> -> Iter<(usize, Instruction)> (only compute idx once)

pub fn assemble(code: &[Instruction]) -> Result<Vec<u16>, AssemblerError> {
    let labels = preparse(code)?;
    let mut idx: usize = 0;
    let mut top_label = String::new();
    let mut result = Vec::new();

    for inst in code {
        match inst {
            Instruction::Label(label) => top_label = label.to_string(),
            Instruction::Sublabel(_) => {},
            Instruction::Include(_) => todo!(),
            Instruction::Define(_, _) => {},
            Instruction::RawWords(words) => {
                let u16s = words.iter().map(|w| w.to_u16(&top_label, &labels)).collect::<Result<Vec<u16>, _>>()?;
                let len = u16s.len();
                result.extend(u16s);
                idx += len;
            },
            Instruction::TwoOp(opcode, lhs, rhs) => {
                let len = 1 + lhs.size() + rhs.size();
                let (lhs, lhs_suffix) = lhs.to_u5(&top_label, &labels)?;
                let (rhs, rhs_suffix) = rhs.to_u5(&top_label, &labels)?;
                result.push(*opcode as u16 | (lhs << 5) | rhs);
                result.extend(lhs_suffix);
                result.extend(rhs_suffix);
                idx += len;
            },
            Instruction::SingleOp(opcode, op) => {
                let len = 1 + op.size();
                let (op, suffix) = op.to_u5(&top_label, &labels)?;
                result.push(*opcode as u16 | op);
                result.extend(suffix);
                idx += len;
            },
            Instruction::NoOp(opcode) => {
                idx += 1;
                result.push(*opcode as u16);
            },
            Instruction::Blank => {},
        }
    }

    Ok(result)
}
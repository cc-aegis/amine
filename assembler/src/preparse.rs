use std::collections::HashMap;
use crate::AssemblerError;
use crate::instruction::Instruction;
use crate::operand::ConstOp;

#[derive(Debug)]
pub struct UnallowedInclude(String);

pub fn preparse(code: &[Instruction]) -> Result<HashMap<String, ConstOp>, AssemblerError> {
    let mut idx: usize = 0;
    let mut top_label = String::new();
    let mut labels = HashMap::new();
    for inst in code {
        match inst {
            Instruction::Label(label) => {
                labels.insert(label.clone(), ConstOp::Value(idx as u16));
                top_label = label.clone();
            }
            Instruction::Sublabel(sublabel) => {
                labels.insert(format!("{top_label}.{sublabel}"), ConstOp::Value(idx as u16));
            }
            Instruction::Include(include) => return Err(AssemblerError::UnallowedInclude(UnallowedInclude(include.clone()))),
            Instruction::Define(label, value) => {
                labels.insert(label.clone(), value.clone());
            }
            Instruction::RawWords(raw) => {
                idx += raw.iter().map(/* ConstOp::size */ |_| 1).sum::<usize>();
            },
            Instruction::TwoOp(_, lhs, rhs) =>{
                idx += 1 + lhs.size() + rhs.size();
            },
            Instruction::SingleOp(_, op) => {
                idx += 1 + op.size();
            },
            Instruction::NoOp(_) => {
                idx += 1;
            },
            Instruction::Blank => {}
        }
    }

    Ok(labels)
}
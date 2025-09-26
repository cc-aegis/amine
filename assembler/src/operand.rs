use std::collections::HashMap;
use std::str::FromStr;
use crate::AssemblerError;

#[derive(Debug)]
pub struct UnallowedRegister;
#[derive(Debug)]
pub struct InvalidOperand(String);
#[derive(Debug)]
pub struct InvalidLabel(String);


#[derive(Debug)]
pub enum RegOp {
    Direct(RawRegOp),
    Indirect(RawRegOp),
}

#[derive(Debug)]
pub enum RawRegOp {
    Const(String),
    Value(u16),
    Register(Register),
}

#[derive(Debug, Clone)]
pub enum ConstOp {
    Const(String),
    Value(u16),
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
    R0, R1, R2, R3, R4, R5, R6, R7,
    RR, RI, RB, RS, RG, RD, RF,
}

impl FromStr for RegOp {
    type Err = InvalidOperand;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (s.starts_with('['), s.ends_with(']')) {
            (true, true) => Ok(RegOp::Indirect(s[1..s.len() - 1].parse()?)),
            (false, false) => Ok(RegOp::Direct(s.parse()?)),
            _ => Err(InvalidOperand(s.to_string())),
        }
    }
}

impl RegOp {
    pub fn size(&self) -> usize {
        match self {
            RegOp::Direct(op) => op.size(),
            RegOp::Indirect(op) => op.size(),
        }
    }

    pub fn to_u5(&self, top_label: &str, labels: &HashMap<String, ConstOp>) -> Result<(u16, Option<u16>), AssemblerError> {
        let (prefix, (op, suffix)) = match self {
            RegOp::Direct(op) => (0x00, op.to_u4(top_label, labels)?),
            RegOp::Indirect(op) => (0x10, op.to_u4(top_label, labels)?),
        };
        Ok((prefix | op, suffix))
    }
}

impl FromStr for RawRegOp {
    type Err = InvalidOperand;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('#') {
            Ok(RawRegOp::Value(parse_val(s)
                .ok_or_else(|| InvalidOperand(s.to_string()))?))
        } else {
            match s {
                "r0" => Ok(RawRegOp::Register(Register::R0)),
                "r1" => Ok(RawRegOp::Register(Register::R1)),
                "r2" => Ok(RawRegOp::Register(Register::R2)),
                "r3" => Ok(RawRegOp::Register(Register::R3)),
                "r4" => Ok(RawRegOp::Register(Register::R4)),
                "r5" => Ok(RawRegOp::Register(Register::R5)),
                "r6" => Ok(RawRegOp::Register(Register::R6)),
                "r7" => Ok(RawRegOp::Register(Register::R7)),
                "rr" => Ok(RawRegOp::Register(Register::RR)),
                "ri" => Ok(RawRegOp::Register(Register::RI)),
                "rb" => Ok(RawRegOp::Register(Register::RB)),
                "rs" => Ok(RawRegOp::Register(Register::RS)),
                "rg" => Ok(RawRegOp::Register(Register::RG)),
                "rd" => Ok(RawRegOp::Register(Register::RD)),
                "rf" => Ok(RawRegOp::Register(Register::RF)),
                _ => Ok(RawRegOp::Const(s.to_string())),
            }
        }
    }
}

impl RawRegOp {
    pub fn size(&self) -> usize {
        match self {
            RawRegOp::Const(_) => 1,
            RawRegOp::Value(_) => 1,
            RawRegOp::Register(_) => 0,
        }
    }

    pub fn to_u4(&self, top_label: &str, labels: &HashMap<String, ConstOp>) -> Result<(u16, Option<u16>), AssemblerError> {
        match self {
            RawRegOp::Const(val) => {
                let op = if val.starts_with('.') {
                    labels.get(&format!("{top_label}{val}"))
                } else {
                    labels.get(val)
                };

                let suffix = op
                    .ok_or_else(|| AssemblerError::InvalidLabel(InvalidLabel(val.to_string())))?
                    .to_u16(top_label, labels)?;

                Ok((0b1111, Some(suffix)))
            }
            RawRegOp::Value(val) => Ok((0b1111, Some(*val))),
            RawRegOp::Register(reg) => Ok((*reg as u16, None))
        }
    }
}

fn parse_val(s: &str) -> Option<u16> {
    s.parse::<u16>()
        .or_else(|_| s.parse::<i16>().map(|i| i as u16))
        .or_else(|_| s.parse::<f16>().map(f16::to_bits))
        .or_else(|_| s.parse::<bool>().map(|b| b as u16))
        .ok()
        .or_else(|| (s == "nullptr").then(|| 0))
}


impl FromStr for ConstOp {
    type Err = AssemblerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match RawRegOp::from_str(s) {
            Ok(RawRegOp::Const(c)) => Ok(ConstOp::Const(c)),
            Ok(RawRegOp::Value(v)) => Ok(ConstOp::Value(v)),
            Ok(RawRegOp::Register(_)) => Err(AssemblerError::UnallowedRegister(UnallowedRegister)),
            Err(err) => Err(AssemblerError::InvalidOperand(err)),
        }
    }
}

impl ConstOp {
    fn parse_struct(s: &str) -> Result<Vec<Self>, AssemblerError> {
        // <single> | [<single>] | {...} (?) |
        // todo: implement others
        //Ok(vec![s.into()?])
        todo!()
    }

    pub fn to_u16(&self, top_label: &str, labels: &HashMap<String, ConstOp>) -> Result<u16, AssemblerError> {
        match self {
            ConstOp::Const(val) => {
                let op = if val.starts_with('.') {
                    labels.get(&format!("{top_label}{val}"))
                } else {
                    labels.get(val)
                };

                op
                    .ok_or_else(|| AssemblerError::InvalidLabel(InvalidLabel(val.to_string())))?
                    .to_u16(top_label, labels)
            }
            ConstOp::Value(val) => Ok(*val),
        }
    }
}
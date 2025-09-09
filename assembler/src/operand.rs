use std::str::FromStr;

#[derive(Debug)]
pub enum Operand {
    Direct(OpValue),
    Indirect(OpValue),
}

#[derive(Debug)]
pub struct InvalidOperand(String);

impl FromStr for Operand {
    type Err = InvalidOperand;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (s.starts_with('['), s.ends_with(']')) {
            (true, true) => Ok(Operand::Indirect(s[1..s.len() - 1].parse()?)),
            (false, false) => Ok(Operand::Direct(s.parse()?)),
            _ => Err(InvalidOperand(s.into())),
        }
    }
}

#[derive(Debug)]
pub enum Register {
    R0, R1, R2, R3, R4, R5, R6, R7,
    RR, RI, RB, RS, RG, RD, RF,
}

#[derive(Debug)]
pub enum OpValue {
    Register(Register),
    U16(u16),
    Constant(String),
}

impl FromStr for OpValue {
    type Err = InvalidOperand;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('#') {
            Ok(OpValue::U16(parse_val(s)
                .ok_or_else(|| InvalidOperand(s.into()))?))
        } else {
            match s {
                "r0" => Ok(OpValue::Register(Register::R0)),
                "r1" => Ok(OpValue::Register(Register::R1)),
                "r2" => Ok(OpValue::Register(Register::R2)),
                "r3" => Ok(OpValue::Register(Register::R3)),
                "r4" => Ok(OpValue::Register(Register::R4)),
                "r5" => Ok(OpValue::Register(Register::R5)),
                "r6" => Ok(OpValue::Register(Register::R6)),
                "r7" => Ok(OpValue::Register(Register::R7)),
                "rr" => Ok(OpValue::Register(Register::RR)),
                "ri" => Ok(OpValue::Register(Register::RI)),
                "rb" => Ok(OpValue::Register(Register::RB)),
                "rs" => Ok(OpValue::Register(Register::RS)),
                "rg" => Ok(OpValue::Register(Register::RG)),
                "rd" => Ok(OpValue::Register(Register::RD)),
                "rf" => Ok(OpValue::Register(Register::RF)),
                _ => Ok(OpValue::Constant(s.into())),
            }
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
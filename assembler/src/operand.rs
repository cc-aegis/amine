use std::str::FromStr;

pub enum Operand {
    Direct(OpValue),
    Indirect(OpValue),
}

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

pub enum OpValue {
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
            Ok(OpValue::Constant(s.into()))
        }
    }
}

fn parse_val(s: &str) -> Option<u16> {
    // TODO: s.parse::<bool>, then chain calls on Err, or return None at end
    if s == "true" {
        Some(1)
    } else if s == "false" {
        Some(0)
    } else if let Ok(u) = s.parse::<u16>() {
        Some(u)
    } else if let Ok(i) = s.parse::<i16>() {
        Some(i as u16)
    } else if let Ok(f) = s.parse::<f16>() {
        Some(f.to_bits())
    } else {
        None
    }
}
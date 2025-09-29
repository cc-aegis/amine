use std::fmt::format;
use crate::cpu::CPU;
use crate::plugin::{Plugin, RegisterInsight};

const ANSI_PURPLE: &'static str = "\x1b[35m";
const ANSI_RESET: &'static str = "\x1b[0m";

pub struct StructInsight {
    registers: [bool; 8],
    depth: usize,
}

/*
+----+-------+-------------------+
|    | addr. | data at location  |
+----+-------+-------------------+
| r0 | 65120 |  4325  4456 12345 |
| r1 |  1230 |   120   213    22 |
| r2 |     4 |     0     1     8 |
| r3 |     0 | 12345 24966   401 |
+----+-------+-------------------+

 */

impl StructInsight {
    pub fn new(registers: [bool; 8], depth: usize) -> Self {
        StructInsight {
            registers,
            depth,
        }
    }
}

impl Plugin for StructInsight {
    fn update(&mut self, cpu: &mut CPU) {
        let data_width = self.depth.max(3) * 6 + 1;
        let mut result = String::new();
        result += &format!("+----+-------+{0}+\n", "-".repeat(data_width));
        result += &format!("|    | addr. | data at location {0}|\n", " ".repeat(data_width - 18));
        result += &format!("+----+-------+{0}+\n", "-".repeat(data_width));
        for (idx, _) in self.registers.iter().enumerate().filter(|(_, p)| **p) {
            let addr = cpu.registers[idx] as usize;
            let mut entries = String::new();
            for idx in 0..self.depth {
                entries += &format!("{: >5} ", cpu.ram[(addr + idx) & 65535]);
            }
            result += &format!("| r{idx} | {addr: >5} | {entries}|\n");
        }
        result += &format!("+----+-------+{0}+", "-".repeat(data_width));

        println!("{ANSI_PURPLE}{result}{ANSI_RESET}")
    }
}
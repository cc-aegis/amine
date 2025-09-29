use std::hash::{DefaultHasher, Hasher};
use crate::cpu::CPU;
use crate::plugin::Plugin;

const ANSI_ORANGE: &'static str = "\x1b[33m";
const ANSI_YELLOW: &'static str = "\x1b[93m";
const ANSI_GREEN: &'static str = "\x1b[92m";
const ANSI_PURPLE: &'static str = "\x1b[35m";
const ANSI_BLUE: &'static str = "\x1b[36m";
const ANSI_RESET: &'static str = "\x1b[0m";

pub const RI: usize = 9;
pub const RS: usize = 11;

pub struct RegisterInsight {
    show_next_instruction: bool,
    show_stack_window: bool,
    show_ram_checksum: bool,
}

impl RegisterInsight {
    pub fn new(show_next_instruction: bool, show_stack_window: bool, show_ram_checksum: bool) -> Self {
        RegisterInsight {
            show_next_instruction,
            show_stack_window,
            show_ram_checksum,
        }
    }
}

fn format_inst(cpu: &CPU) -> String {
    let inst = cpu.ram[cpu.registers[RI] as usize];

    format!(
        " {ANSI_GREEN}({0:0>6b}-{1:0>5b}-{2:0>5b}){ANSI_RESET}",
        (inst >> 10),
        (inst >> 5) & 0x1F,
        inst & 0x1F,
    )
}

fn format_stack(cpu: &CPU) -> String {
    let idx = cpu.registers[RS];
    format!(
        " {ANSI_PURPLE}[{0} {1} >{2}< {3} {4}]{ANSI_RESET}",
        cpu.ram[idx.wrapping_sub(2) as usize],
        cpu.ram[idx.wrapping_sub(1) as usize],
        cpu.ram[idx as usize],
        cpu.ram[idx.wrapping_add(1) as usize],
        cpu.ram[idx.wrapping_add(2) as usize],
    )
}

fn format_ram_checksum(cpu: &CPU) -> String {
    let mut hasher = DefaultHasher::new();
    for val in cpu.ram.as_slice() {
        hasher.write_u16(*val);
    }
    let checksum = hasher.finish();
    format!(
        " {ANSI_BLUE}#{checksum:x}{ANSI_RESET}",
    )
}

impl Plugin for RegisterInsight {
    fn update(&mut self, cpu: &mut CPU) {
        // TODO: add rr?
        println!(
            "{ANSI_ORANGE}{0}: {ANSI_YELLOW}{1:?}{2}{3}{4}{5:?}",
            cpu.cycle,
            &cpu.registers,
            if self.show_next_instruction { &format_inst(cpu) } else { "" },
            if self.show_stack_window { &format_stack(cpu) } else { "" },
            if self.show_ram_checksum { &format_ram_checksum(cpu) } else { "" },
            &cpu.ram[..8]
        );
    }
}
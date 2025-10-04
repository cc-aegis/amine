mod config;

use std::hash::{DefaultHasher, Hasher};
use std::path::Path;
use emulator::cpu::CPU;
use emulator::device::Display;
use emulator::plugin::{ClearDbg, Corruption, LimitClockSpeed, PrintDbg, RamView, RegisterInsight, StructInsight};
use crate::config::AmineConfig;
// TODO: amine build file.s
// TODO: amine build -p path
// TODO: amine run file.x -D "devices.." -P "plugins.."

fn checksum(data: &[u16]) -> u64 {
    let mut hasher = DefaultHasher::new();
    for item in data {
        hasher.write_u16(*item);
    }
    hasher.finish()
}

/*
amine run -F
amine run -D
amine run -P (default)
 */

fn main() {
    let (mut cpu, upc) = AmineConfig::import(".")
        .unwrap()
        .create();

    loop {
        cpu.update(upc);
    }
}

// TODO: plugin that display 256x256 of values (black to white) in separate window
// TODO: option: use colors to signal owner (mem)
// TODO: option: highlight which parts belong to the some block somehow
// TODO: option: show registers above
// TODO: option: heatmap
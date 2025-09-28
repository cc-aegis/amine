use std::hash::{DefaultHasher, Hasher};
use std::path::Path;
use emulator::cpu::CPU;
use emulator::device::Display;
use emulator::plugin::{ClearDbg, Corruption, LimitClockSpeed, PrintDbg, RegisterInsight};
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

fn main() {
    // struct CPUPlugin { freq: usize, act: fn(&mut CPU) }
    // Thread { cpu: CPU, plugins: Vec<CPUPlugin> }
    // run loop of .next(); every 1024 steps call all plugins (plugins like interrupt, reading dbg, limitops (2 mops))

    let bytecode = assembler::assemble_project(&Path::new("../projects/recursive_scheduler")).unwrap();
    let len = bytecode
        .iter()
        .enumerate()
        .fold(0, |result, (idx, word)| match word {
            0 => result,
            _ => idx + 1,
        });
    println!("{0:?}", &bytecode[..len]);
    println!("word count: {len} ({0:.2}% of RAM)", len as f32 / 655.36);
    println!("checksum: {}", checksum(bytecode.as_slice()));
    let mut cpu = CPU::from(bytecode);

    // cpu.attach(Box::new(Display::new()));
    // cpu.attach(Box::new(Display::new()));

    cpu.install(Box::new(PrintDbg));
    // cpu.install(Box::new(Corruption::new(500_000)));
    // cpu.install(Box::new(RegisterInsight::new(true, true, true)));
    cpu.install(Box::new(LimitClockSpeed::new(20_000_000))); // 20_000_000

    loop {
        cpu.update(65536);
    }


    todo!("collect args")
}

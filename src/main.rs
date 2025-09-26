use std::path::Path;
use emulator::cpu::CPU;

fn main() {
    let mut ram = Box::new([0; 65536]);
    ram[0] = 0x002F;
    ram[1] = 0x1234;
    let mut cpu = CPU::from(ram);

    // struct CPUPlugin { freq: usize, act: fn(&mut CPU) }
    // Thread { cpu: CPU, plugins: Vec<CPUPlugin> }
    // run loop of .next(); every 1024 steps call all plugins (plugins like interrupt, reading dbg, limitops (2 mops))
    cpu.next();

    let code = assembler::link(&Path::new("./examples/mem/"));
    dbg!(&code);
    assembler::assemble(&code.unwrap());



    todo!("collect args")
}

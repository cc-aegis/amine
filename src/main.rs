use std::path::Path;
use emulator::cpu::CPU;

fn main() {
    let mut ram = Box::new([0; 65536]);
    ram[0] = 0x002F;
    ram[1] = 0x1234;
    let mut cpu = CPU::from(ram);
    cpu.next();

    let code = assembler::link(&Path::new("./code/"));
    dbg!(code);


    todo!("collect args")
}

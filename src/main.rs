use std::path::Path;
use emulator::cpu::CPU;

// TODO: amine build file.s
// TODO: amine build -p path
// TODO: amine run file.x

fn main() {
    // struct CPUPlugin { freq: usize, act: fn(&mut CPU) }
    // Thread { cpu: CPU, plugins: Vec<CPUPlugin> }
    // run loop of .next(); every 1024 steps call all plugins (plugins like interrupt, reading dbg, limitops (2 mops))

    let bytecode = assembler::assemble_file(&Path::new("./examples/fib/main.s")).unwrap();
    let mut cpu = CPU::from(bytecode);

    loop {
        cpu.next();
    }


    todo!("collect args")
}

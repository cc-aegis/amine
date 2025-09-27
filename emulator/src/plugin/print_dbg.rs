use crate::cpu::CPU;
use crate::plugin::Plugin;

const ANSI_YELLOW: &'static str = "\x1b[93m";
const ANSI_BLUE: &'static str = "\x1b[94m";
const ANSI_RESET: &'static str = "\x1b[0m";

// TODO (orange): (<idx>:<cycle>) ...

pub struct PrintDbg;

impl Plugin for PrintDbg {
    fn update(&mut self, cpu: &mut CPU) {
        while let Some(out) = cpu.dbg_queue.pop_back() {
            println!(
                "{ANSI_YELLOW}dbg: {ANSI_BLUE}{:?}{ANSI_YELLOW} (u16) or {ANSI_BLUE}{:?}{ANSI_YELLOW} (f16){ANSI_RESET}",
                out,
                unsafe { std::mem::transmute::<u16, f16>(out) },
            )
        }
    }
}
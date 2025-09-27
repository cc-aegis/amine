use crate::cpu::CPU;
use crate::plugin::Plugin;


pub struct ClearDbg;

impl Plugin for ClearDbg {
    fn update(&mut self, cpu: &mut CPU) {
        cpu.dbg_queue.clear();
    }
}
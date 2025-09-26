mod print_dbg;
mod limit_clock_speed;

use crate::cpu::CPU;
pub use print_dbg::PrintDbg;
pub use limit_clock_speed::LimitClockSpeed;

pub trait Plugin {
    fn run(&mut self, cpu: &mut CPU);
}
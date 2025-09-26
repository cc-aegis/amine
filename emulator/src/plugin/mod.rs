mod print_dbg;
mod limit_clock_speed;
mod register_insight;

use crate::cpu::CPU;
pub use print_dbg::PrintDbg;
pub use limit_clock_speed::LimitClockSpeed;
pub use register_insight::RegisterInsight;

pub trait Plugin {
    fn run(&mut self, cpu: &mut CPU);
}
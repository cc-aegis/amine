mod print_dbg;
mod limit_clock_speed;
mod register_insight;
mod clear_dbg;

use crate::cpu::CPU;
pub use print_dbg::PrintDbg;
pub use limit_clock_speed::LimitClockSpeed;
pub use register_insight::RegisterInsight;
pub use clear_dbg::ClearDbg;

pub trait Plugin {
    fn update(&mut self, cpu: &mut CPU);
}
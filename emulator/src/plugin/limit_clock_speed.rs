use std::time::Instant;
use crate::cpu::CPU;
use crate::plugin::Plugin;

pub struct LimitClockSpeed {
    ips: usize,
    start: Instant,
}

impl LimitClockSpeed {
    pub fn new(ips: usize) -> Self {
        LimitClockSpeed {
            ips,
            start: Instant::now(),
        }
    }
}

impl Plugin for LimitClockSpeed {
    fn run(&mut self, cpu: &mut CPU) {
        let passed_time = Instant::now() - self.start;
        let cycle_ceil = passed_time.as_micros() as usize * self.ips / 1000000;
        let cycle_diff = cpu.cycle as isize - cycle_ceil as isize;
        if cycle_diff > 0 {
            let diff_time = cycle_diff as f64 / self.ips as f64;
            std::thread::sleep(std::time::Duration::from_micros((diff_time * 1000000.0) as u64));
        }
    }
}
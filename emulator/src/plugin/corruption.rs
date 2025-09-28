use std::time::Instant;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::cpu::CPU;
use crate::plugin::Plugin;

pub struct Corruption {
    instructions_per_flip: usize,
    next_flip: usize,
    rng: ThreadRng,
}

impl Corruption {
    pub fn new(instructions_per_flip: usize) -> Self {
        Corruption {
            instructions_per_flip,
            next_flip: instructions_per_flip,
            rng: rand::rng(),
        }
    }
}

impl Plugin for Corruption {
    fn update(&mut self, cpu: &mut CPU) {
        while self.next_flip < cpu.cycle {
            self.next_flip += self.instructions_per_flip;

            let idx: u16 = self.rng.random();
            let bit: u16 = 1 << (self.rng.random::<u16>() & 15);

            cpu.ram[idx as usize] ^= bit;
        }
    }
}
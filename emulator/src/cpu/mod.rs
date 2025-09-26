mod next;
mod registers;
mod opcodes;

use std::collections::VecDeque;
use crate::device::Device;
use crate::plugin::Plugin;

pub struct CPU {
    pub ram: Box<[u16; 65536]>,
    pub registers: [u16; 15],
    pub io: Vec<Box<dyn Device>>,
    pub dbg_queue: VecDeque<u16>,
    plugins: Vec<Box<dyn Plugin>>,
    pub cycle: usize,
}

impl CPU {
    pub fn from(ram: Box<[u16; 65536]>) -> Self {
        CPU {
            ram,
            registers: [0; 15],
            io: Vec::new(),
            dbg_queue: VecDeque::new(),
            plugins: Vec::new(),
            cycle: 0,
        }
    }

    pub fn attach(&mut self, device: impl Into<Box<dyn Device>>) {
        self.io.push(device.into());
    }

    pub fn install(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
}
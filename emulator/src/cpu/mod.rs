mod next;
mod registers;
mod opcodes;

use crate::device::Device;

pub struct CPU {
    ram: Box<[u16; 65536]>,
    registers: [u16; 15],
    io: Vec<Box<dyn Device>>,
}

impl CPU {
    pub fn from(ram: Box<[u16; 65536]>) -> Self {
        CPU {
            ram,
            registers: [0; 15],
            io: Vec::new(),
        }
    }

    pub fn attach(&mut self, device: impl Into<Box<dyn Device>>) {
        self.io.push(device.into());
    }
}
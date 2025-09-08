use crate::cpu::CPU;
use crate::cpu::opcodes as O;
use crate::cpu::registers as R;

macro_rules! post_inc {
    ($exp:expr) => { { let result = $exp; $exp = $exp.wrapping_add(1); result } };
}

macro_rules! pre_dec {
    ($exp:expr) => { { $exp = $exp.wrapping_sub(1); $exp } };
}

macro_rules! push {
    ($self:expr, $val:expr) => { $self.ram[$self.registers[R::RG].wrapping_add(post_inc!($self.registers[R::RS])) as usize] = $val; };
}

macro_rules! pop {
    ($self:expr) => { $self.ram[$self.registers[R::RG].wrapping_add(pre_dec!($self.registers[R::RS])) as usize] };
}

macro_rules! deref {
    ($self:expr, $addr:expr) => { $self.ram[$self.registers[R::RG].wrapping_add($addr) as usize] };
}

unsafe fn make_mut<T>(ptr: &T) -> &mut T {
    #[allow(mutable_transmutes)]
    unsafe { std::mem::transmute(ptr) }
}

impl CPU {
    fn read_word(&mut self) -> u16 {
        self.ram[post_inc!(self.registers[R::RI]) as usize]
    }

    // TODO: fix whatever this is
    unsafe fn ptr(&mut self, operand: u16) -> &mut u16 {
        #[allow(mutable_transmutes)]
        let cpu = unsafe { make_mut(self) };
        unsafe {
            let address = match operand & 0x000F {
                0..15 => &mut cpu.registers[(operand & 0x000F) as usize],
                15 => {
                    let idx = cpu.registers[9];
                    cpu.registers[9] = idx + 1;
                    &mut cpu.ram[idx.wrapping_add(cpu.registers[R::RR]) as usize]
                },
                _ => unreachable!(),
            } as *mut u16;

            &mut *(if (operand & 0x0010) != 0 {
                let idx = *address;
                &mut cpu.ram[idx.wrapping_add(cpu.registers[R::RB]) as usize]
            } else {
                address
            })
        }
    }

    fn next(&mut self) {
        let inst = self.read_word();
        if inst & 0xFC00 != 0 {
            let op1 = unsafe { make_mut(self).ptr((inst >> 5) & 0x001F) };
            let op2 = unsafe { make_mut(self).ptr(inst & 0x001F) };
            unsafe { make_mut(self) }.exec_two_op(inst & 0xFC00, op1, op2);
        } else if inst & 0x03D0 != 0 {
            let op = unsafe { make_mut(self).ptr(inst & 0x001F) };
            unsafe { make_mut(self) }.exec_single_op(inst & 0x03D0, op);
        } else {
            self.exec_no_op(inst & 0x001F);
        }
    }

    fn exec_two_op(&mut self, opcode: u16, op1: &mut u16, op2: &mut u16) {
        match opcode {
            O::OPCODE_MOV => *op1 = *op2,
            O::OPCODE_PUSHT => {
                push!(self, *op1);
                push!(self, *op2);
            },
            O::OPCODE_POPT => {
                *op2 = pop!(self);
                *op1 = pop!(self);
            },
            O::OPCODE_READ => *op1 = deref!(self, *op2),
            O::OPCODE_WRITE => deref!(self, *op1) = *op2,
            O::OPCODE_COPY => deref!(self, *op1) = deref!(self, *op2),
            O::OPCODE_SWAP => std::mem::swap(op1, op2),
            O::OPCODE_READITR => *op1 = deref!(self, post_inc!(*op2)),
            O::OPCODE_WRITEITR => deref!(self, post_inc!(*op1)) = *op2,
            _ => panic!("opcode {opcode} for two-op instruction not expected"),
        }
    }

    fn exec_single_op(&mut self, opcode: u16, op: &mut u16) {}

    fn exec_no_op(&mut self, opcode: u16) {
        match opcode {
            O::OPCODE_NOP => {},
            O::OPCODE_RET => todo!(),
            O::OPCODE_SEND => todo!(),
            O::OPCODE_EXIT => todo!(),
            _ => panic!("opcode {opcode} for no-op instruction not expected"),
        }
    }
}
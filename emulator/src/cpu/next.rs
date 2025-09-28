use crate::cpu::CPU;
use crate::cpu::opcodes as O;
use crate::cpu::registers as R;

macro_rules! add {
    ($lhs:expr, $rhs:expr) => { $lhs.wrapping_add($rhs) };
}

macro_rules! sub {
    ($lhs:expr, $rhs:expr) => { $lhs.wrapping_sub($rhs) };
}

macro_rules! mul {
    ($lhs:expr, $rhs:expr) => { $lhs.wrapping_mul($rhs) };
}

macro_rules! div {
    ($lhs:expr, $rhs:expr) => { $lhs.wrapping_div($rhs) };
}

macro_rules! post_inc {
    ($exp:expr) => { { let result = $exp; $exp = add!($exp, 1); result } };
}

macro_rules! pre_dec {
    ($exp:expr) => { { $exp = sub!($exp, 1); $exp } };
}

macro_rules! post_dec {
    ($exp:expr) => { { let result = $exp; $exp = sub!($exp, 1); result } };
}

macro_rules! push {
    ($self:expr, $val:expr) => { { $self.ram[add!($self.registers[R::RG], post_inc!($self.registers[R::RS])) as usize] = $val; } };
}

macro_rules! pop {
    ($self:expr) => { $self.ram[add!($self.registers[R::RG], pre_dec!($self.registers[R::RS])) as usize] };
}

macro_rules! deref {
    ($self:expr, $addr:expr) => { $self.ram[add!($self.registers[R::RG], $addr) as usize] };
}

macro_rules! jump {
    ($self:expr, $addr:expr) => { $self.registers[R::RI] = add!($addr, $self.registers[R::RR]) };
}

macro_rules! f16 {
    ($it:expr) => { f16::from_bits($it) };
}

macro_rules! u16 {
    ($it:expr) => { $it.to_bits() };
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
                    let idx = cpu.registers[R::RI];
                    cpu.registers[R::RI] = idx + 1;
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

    #[inline(always)]
    fn next(&mut self) {
        let inst = self.read_word();
        if inst & 0xFC00 != 0 {
            let op1 = unsafe { make_mut(self).ptr((inst >> 5) & 0x001F) };
            let op2 = unsafe { make_mut(self).ptr(inst & 0x001F) };
            unsafe { make_mut(self) }.exec_two_op(inst & 0xFC00, op1, op2);
        } else if inst & 0x03E0 != 0 {
            let op = unsafe { make_mut(self).ptr(inst & 0x001F) };
            unsafe { make_mut(self) }.exec_single_op(inst & 0x03E0, op);
        } else {
            self.exec_no_op(inst & 0x001F);
        }
    }

    pub fn update(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.next();
        }

        self.cycle += iterations;

        for plugin in unsafe { make_mut(&self.plugins) } {
            plugin.update(unsafe { make_mut(self) })
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
            O::OPCODE_COPYITR => deref!(self, post_inc!(*op1)) = deref!(self, post_inc!(*op2)),
            O::OPCODE_LOOKUP => *op1 = deref!(self, add!(*op1, *op2)),
            O::OPCODE_JLOOKUP => jump!(self, deref!(self, add!(*op1, *op2))),
            O::OPCODE_CLOOKUP => todo!(),
            O::OPCODE_JRNZDEC => if post_dec!(*op1) != 0 { jump!(self, *op2); },
            O::OPCODE_CALLW => todo!(),

            O::OPCODE_JRZ => if *op1 == 0 { jump!(self, *op2); },
            O::OPCODE_JRNZ => if *op1 != 0 { jump!(self, *op2); },
            O::OPCODE_JRGT => if (*op1 as i16) > 0 { jump!(self, *op2); },
            O::OPCODE_JRLT => if (*op1 as i16) < 0 { jump!(self, *op2); },

            O::OPCODE_ADD => *op1 = add!(*op1, *op2),
            O::OPCODE_SUB => *op1 = sub!(*op1, *op2),
            O::OPCODE_MUL => *op1 = mul!(*op1, *op2),
            O::OPCODE_DIV => *op1 = div!(*op1, *op2),
            O::OPCODE_UTOF => *op1 = u16!(*op2 as f16),
            O::OPCODE_ITOF => *op1 = u16!(*op2 as i16 as f16),
            O::OPCODE_IMUL => *op1 = mul!(*op1 as i16, *op2 as i16) as u16,
            O::OPCODE_IDIV => *op1 = div!(*op1 as i16, *op2 as i16) as u16,
            O::OPCODE_FADD => *op1 = u16!(f16!(*op1) + f16!(*op2)),
            O::OPCODE_FSUB => *op1 = u16!(f16!(*op1) - f16!(*op2)),
            O::OPCODE_FMUL => *op1 = u16!(f16!(*op1) * f16!(*op2)),
            O::OPCODE_FDIV => *op1 = u16!(f16!(*op1) / f16!(*op2)),
            O::OPCODE_FTOU => *op1 = f16!(*op2) as u16,
            O::OPCODE_FTOI => *op1 = f16!(*op2) as i16 as u16,

            O::OPCODE_AND => *op1 &= *op2,
            O::OPCODE_OR => *op1 |= *op2,
            O::OPCODE_XOR => *op1 ^= *op2,

            O::OPCODE_SHL => *op1 <<= *op2,
            O::OPCODE_SHR => *op1 >>= *op2,

            O::OPCODE_CTX => if let Some(device) = self.io.get_mut(self.registers[R::RD] as usize) {
                device.set_context(*op1, *op2);
            }

            _ => panic!("opcode {opcode} for two-op instruction not expected"),
        }
    }

    fn exec_single_op(&mut self, opcode: u16, op: &mut u16) {
        match opcode {
            O::OPCODE_DBG => self.dbg_queue.push_front(*op),
            O::OPCODE_PUSH => push!(self, *op),
            O::OPCODE_POP => *op = pop!(self),

            O::OPCODE_INC => *op = add!(*op, 1),
            O::OPCODE_DEC => *op = sub!(*op, 1),
            O::OPCODE_SSHR => *op >>= 1,

            O::OPCODE_FLOOR => *op = u16!(f16!(*op).floor()),
            O::OPCODE_CEIL => todo!(),

            O::OPCODE_JMP => jump!(self, *op),
            O::OPCODE_CALL => {
                push!(self, self.registers[R::RB]);
                push!(self, self.registers[R::RI]);
                self.registers[R::RB] = self.registers[R::RS];
                jump!(self, *op);
            },

            _ => panic!("opcode {opcode} for single-op instruction not expected"),
        }
    }

    fn exec_no_op(&mut self, opcode: u16) {
        match opcode {
            O::OPCODE_NOP => {},
            O::OPCODE_RET => {
                self.registers[R::RS] = self.registers[R::RB];
                self.registers[R::RI] = pop!(self);
                self.registers[R::RB] = pop!(self);
            },
            O::OPCODE_SEND =>  if let Some(device) = self.io.get_mut(self.registers[R::RD] as usize) {
                device.send();
            },

            O::OPCODE_EXIT => panic!("exit"),

            _ => panic!("opcode {opcode} for no-op instruction not expected"),
        }
    }
}
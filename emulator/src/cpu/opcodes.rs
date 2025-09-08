//two-param instructions
pub const OPCODE_MOV: u16 = 0x0400;
pub const OPCODE_PUSHT: u16 = 0x0800;
pub const OPCODE_POPT: u16 = 0x0C00;
pub const OPCODE_READ: u16 = 0x1000;
pub const OPCODE_WRITE: u16 = 0x1400;
pub const OPCODE_COPY: u16 = 0x1800;
pub const OPCODE_SWAP: u16 = 0x1C00;
pub const OPCODE_READITR: u16 = 0x2000;
pub const OPCODE_WRITEITR: u16 = 0x2400;
pub const OPCODE_COPYITR: u16 = 0x2800;

pub const OPCODE_LOOKUP: u16 = 0x3C00;
pub const OPCODE_JLOOKUP: u16 = 0x4000;
pub const OPCODE_CLOOKUP: u16 = 0x4400;
pub const OPCODE_JRNZDEC: u16 = 0x4800;
pub const OPCODE_CALLW: u16 = 0x4C00;

pub const OPCODE_JRZ: u16 = 0x6000;
pub const OPCODE_JRNZ: u16 = 0x6400;
pub const OPCODE_JRGT: u16 = 0x6800;
pub const OPCODE_JRGE: u16 = 0x6C00;
pub const OPCODE_JRLT: u16 = 0x7000;
pub const OPCODE_JRLE: u16 = 0x7400;

pub const OPCODE_ADD: u16 = 0x8000;
pub const OPCODE_SUB: u16 = 0x8400;
pub const OPCODE_MUL: u16 = 0x8800;
pub const OPCODE_DIV: u16 = 0x8C00;
pub const OPCODE_UTOF: u16 = 0x9000;
pub const OPCODE_ITOF: u16 = 0x9400;
pub const OPCODE_IMUL: u16 = 0x9800;
pub const OPCODE_IDIV: u16 = 0x9C00;
pub const OPCODE_FADD: u16 = 0xA000;
pub const OPCODE_FSUB: u16 = 0xA400;
pub const OPCODE_FMUL: u16 = 0xA800;
pub const OPCODE_FDIV: u16 = 0xAC00;
pub const OPCODE_FTOU: u16 = 0xB000;
pub const OPCODE_FTOI: u16 = 0xB400;

pub const OPCODE_AND: u16 = 0xC000;
pub const OPCODE_OR: u16 = 0xC400;
pub const OPCODE_XOR: u16 = 0xC800;

pub const OPCODE_INV: u16 = 0xD400;
pub const OPCODE_BOOL: u16 = 0xD800;
pub const OPCODE_NEG: u16 = 0xDC00;
pub const OPCODE_SHL: u16 = 0xE000;
pub const OPCODE_SHR: u16 = 0xE400;

pub const OPCODE_CMP: u16 = 0xF800;
pub const OPCODE_CTX: u16 = 0xFC00;


//single-param instructions
pub const OPCODE_DBG: u16 = 0x0020;
pub const OPCODE_PUSH: u16 = 0x0040;
pub const OPCODE_POP: u16 = 0x0060;
pub const OPCODE_CMPZ: u16 = 0x0080;

pub const OPCODE_INC: u16 = 0x0100;
pub const OPCODE_DEC: u16 = 0x0120;
pub const OPCODE_SSHL: u16 = 0x0140;
pub const OPCODE_SSHR: u16 = 0x0160;

pub const OPCODE_FLOOR: u16 = 0x01C0;
pub const OPCODE_CEIL: u16 = 0x01E0;

pub const OPCODE_JZ: u16 = 0x0200;
pub const OPCODE_JNZ: u16 = 0x0220;
pub const OPCODE_JGT: u16 = 0x0240;
pub const OPCODE_JGE: u16 = 0x0260;
pub const OPCODE_JLT: u16 = 0x0280;
pub const OPCODE_JLE: u16 = 0x02A0;


pub const OPCODE_JMP: u16 = 0x0300;
pub const OPCODE_CALL: u16 = 0x0320;

pub const OPCODE_RETV: u16 = 0x0380;
pub const OPCODE_RETL: u16 = 0x03A0;



//no-param instructions
pub const OPCODE_NOP: u16 = 0x0000;
pub const OPCODE_RET: u16 = 0x0001;
pub const OPCODE_SEND: u16 = 0x0002;

pub const OPCODE_EXIT: u16 = 0x001F;
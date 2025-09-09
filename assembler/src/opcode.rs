pub enum TwoOpOpcode {
    Mov = 0x0400,
    PushT = 0x0800,
    PopT = 0x0C00,
    Read = 0x1000,
    Write = 0x1400,
    Copy = 0x1800,
    Swap = 0x1C00,
    ReadItr = 0x2000,
    WriteItr = 0x2400,
    CopyItr = 0x2800,

}

pub enum SingleOpOpcode {}

pub enum NoOpOpcode {}
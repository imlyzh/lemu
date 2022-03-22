use modular_bitfield::prelude::*;



#[bitfield(bits = 32)]
pub struct Satp32 {
    pub mode: B1,
    pub asid: B9,
    pub ppn: B22,
}

#[bitfield(bits = 64)]
pub struct Satp {
    pub mode: B4,
    pub asid: B16,
    pub ppn: B44,
}

pub mod SatpModeValue {
    pub const BARE: u8 = 0;
    pub const SV32: u8 = 1;
    pub const SV39: u8 = 8;
    pub const SV48: u8 = 9;
    pub const SV57: u8 = 10;
    pub const SV64: u8 = 11;
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SatpMode {
    Bare = SatpModeValue::BARE,
    Sv32 = SatpModeValue::SV32,
    Sv39 = SatpModeValue::SV39,
    Sv48 = SatpModeValue::SV48,
    Sv57 = SatpModeValue::SV57,
    Sv64 = SatpModeValue::SV64,
}
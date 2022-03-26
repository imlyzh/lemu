use modular_bitfield::prelude::*;



#[repr(u8)]
#[derive(BitfieldSpecifier)]
#[bits=2]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TVMode {
    Direct = 0,
    Vectored = 1,
}

#[bitfield(bits = 64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stvec {
    pub base: B62,
    #[bits=2]
    pub mode: TVMode,
}

impl Stvec {
    pub fn base_addr(&self) -> u64 {
        (self.base() as u64) << 2
    }
}

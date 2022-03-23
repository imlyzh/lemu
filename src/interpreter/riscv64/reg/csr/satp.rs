use modular_bitfield::prelude::*;



#[repr(u8)]
#[derive(BitfieldSpecifier)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SatpMode32 {
    Bare = 0,
    Sv32 = 1,
}

#[bitfield(bits = 32)]
pub struct Satp32 {
    #[bits=1]
    pub mode: SatpMode32,
    pub asid: B9,
    pub ppn: B22,
}


#[repr(u8)]
#[derive(BitfieldSpecifier)]
#[bits=4]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SatpMode {
    Bare = 0,
    // Sv32 = 1,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
    Sv64 = 11,
}

#[bitfield(bits = 64)]
pub struct Satp {
    #[bits=4]
    pub mode: SatpMode,
    pub asid: B16,
    pub ppn: B44,
}

impl Satp {
    pub fn root_addr(&self) -> u64 {
        (self.ppn() as u64) << 12
    }
}

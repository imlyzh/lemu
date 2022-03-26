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

/*
impl Satp32 {
    pub fn root_addr(&self) -> u64 {
        (self.ppn() as u64) << 12
    }
}
 */

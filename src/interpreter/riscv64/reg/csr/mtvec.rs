use modular_bitfield::prelude::*;

use crate::interpreter::riscv64::irq::RawTrapType;



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
pub struct Tvec {
    pub base: B62,
    #[bits=2]
    pub mode: TVMode,
}

impl Tvec {
    pub fn base_addr(&self) -> u64 {
        (self.base() as u64) << 2
    }

    pub fn get_pc(&self, trap_type: RawTrapType, cause: u64) -> u64 {
        let addr = self.base_addr();
        if self.mode() == TVMode::Direct || trap_type == RawTrapType::Exception {
            addr
        } else {
            addr + 4 * cause
        }
    }
}

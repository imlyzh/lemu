pub mod satp;
pub mod mstatus;
pub mod mtvec;
pub mod mideleg;
pub mod medeleg;
pub mod mcause;
pub mod mie;

use std::cell::RefCell;

use super::{Reg, XLEN, CSRMap};

// pub const CSR_SIZE: usize = 0xD9CF;
pub const CSR_SIZE: usize = 4096;

#[derive(Debug, Clone)]
pub struct CSR(RefCell<[Reg; CSR_SIZE]>);


#[repr(u8)]
#[derive(Debug, Clone)]
pub enum BaseISA {
    RV32I = 1,
    RV64I = 2,
    RV128I = 3,
}

pub const fn base_misa(i: BaseISA) -> u64 {
    (i as u64) << 62
}

pub const fn misa_flag(x: u8) -> u64 {
    0b1 << (x - b'a')
}

// const marchid64: u64 = 0;
// const mimpid: u64 = 0;


impl CSR {

    #[inline]
    pub fn new(misa: u64, hart_id: u64) -> CSR {
        let r = CSR(RefCell::new([0; CSR_SIZE]));
        // r.store(CSRMap::MARCHID, marchid64);
        // r.store(CSRMap::MIMPID, mimpid);
        r.store(CSRMap::MISA, misa);
        r.store(CSRMap::MHARTID, hart_id as Reg);
        // todo
        r
    }

    #[inline]
    pub fn read(&self, reg: usize) -> Reg {
        self.0.borrow()[reg]
    }

    #[inline]
    pub fn store(&self, reg: usize, value: XLEN) {
        if reg != 0 {
            self.0.borrow_mut()[reg] = value;
        }
    }

}

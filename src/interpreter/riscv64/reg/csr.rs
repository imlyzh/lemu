pub mod satp;
pub mod mstatus;
pub mod mtvec;
pub mod mideleg;
pub mod medeleg;
pub mod mcause;
pub mod mie_mip;

use std::cell::RefCell;

use self::{mstatus::MStatus, mie_mip::{Mie, Mip}};

use super::{Reg, Xlen, csrmap};

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
        r.store(csrmap::MISA, misa);
        r.store(csrmap::MHARTID, hart_id as Reg);
        let mstatus = MStatus::new()
            .with_mie(1)
            .with_mpie(1);
        let mie = Mie::new()
            .with_msie(1);
        let mip = Mip::new()
            .with_msie(1);
        r.store(csrmap::MSTATUS, u64::from_le_bytes(mstatus.into_bytes()));
        r.store(csrmap::MIE, u64::from_le_bytes(mie.into_bytes()));
        r.store(csrmap::MIP, u64::from_le_bytes(mip.into_bytes()));

        // todo
        r
    }

    #[inline]
    pub fn read(&self, reg: usize) -> Reg {
        self.0.borrow()[reg]
    }

    #[inline]
    pub fn store(&self, reg: usize, value: Xlen) {
        if reg != 0 {
            self.0.borrow_mut()[reg] = value;
        }
    }

}

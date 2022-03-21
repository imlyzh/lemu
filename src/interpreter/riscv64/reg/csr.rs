use std::cell::RefCell;

use super::{Reg, XLEN};

// pub const CSR_SIZE: usize = 0xD9CF;
pub const CSR_SIZE: usize = 2^12;

#[derive(Debug, Clone)]
pub struct CSR(RefCell<[Reg; CSR_SIZE]>);

impl CSR {

    #[inline]
    pub fn new() -> CSR {
        let r = CSR(RefCell::new([0; CSR_SIZE]));
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

use std::cell::RefCell;

use super::{Reg, XLEN};


#[derive(Debug, Clone)]
pub struct GPR(RefCell<[Reg; 32]>);

impl GPR {
    #[inline]
    pub fn new() -> GPR {
        GPR(RefCell::new([0; 32]))
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
use std::cell::Cell;

use super::{Reg, XLEN};



#[derive(Debug, Clone)]
pub struct PC(Cell<Reg>);

impl PC {
    pub fn new(start_addr: usize) -> PC {
        PC(Cell::new(start_addr as Reg))
    }

    #[inline(always)]
    pub fn store(&self, pc: Reg) {
        self.0.set(pc);
    }

    #[inline(always)]
    pub fn read(&self) -> XLEN {
        self.0.get()
    }
}
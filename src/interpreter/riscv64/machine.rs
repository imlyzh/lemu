use std::cell::{Cell, RefCell};

use crate::abstract_machine::RegInfo;

use super::reg::{REG_MAP, RegType};

pub type XLEN = u64;

pub type Reg = XLEN;

pub struct MachineModel {
    pub gpr: RefCell<[Reg; 32]>,
    pub fgpr: RefCell<[Reg; 32]>,
    pub pc: Cell<Reg>,
    pub csr: Cell<[Reg; 2^12]>,
}

impl MachineModel {
    #[inline]
    pub fn new() -> MachineModel {
        MachineModel {
            gpr: RefCell::new([0; 32]),
            fgpr: RefCell::new([0; 32]),
            pc: Cell::new(0),
            csr: RefCell::new([0; 32]),
        }
    }

    #[inline(always)]
    pub fn read_gpr(&self, reg: usize) -> Reg {
        self.gpr.borrow()[reg]
    }

    #[inline(always)]
    pub fn store_gpr(&self, reg: usize, value: XLEN) {
        if reg != 0 {
            self.gpr.borrow_mut()[reg] = value;
        }
    }

    #[inline(always)]
    pub fn set_pc(&self, pc: Reg) {
        self.pc.set(pc);
    }

    #[inline(always)]
    pub fn read_pc(&self) -> XLEN {
        self.pc.get()
    }

}

impl RegInfo for MachineModel {
    #[inline]
    fn get_reg_value(&self, reg: &str) -> Option<u64> {
        match reg {
            "pc" => Some(self.pc.get()),
            "csr" => Some(self.csr.get()),
            _ => {
                if let Ok(x) = reg.parse::<usize>() {
                    return if x < 32 {
                        Some(self.gpr.borrow()[x])
                    } else {
                        None
                    };
                }
                let map = &REG_MAP;
                let (rt, r) = map.get(reg)?;
                if rt == &RegType::GPR {
                    Some(self.read_gpr(*r))
                } else {
                    Some(self.fgpr.borrow()[*r])
                }
            },
        }
    }
}


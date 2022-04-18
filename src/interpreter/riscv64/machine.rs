use std::cell::Cell;

use crate::abstract_machine::RegInfo;

use super::reg::{REG_MAP, RegType, csr::{CSR, base_misa, BaseISA, misa_flag, mstatus::MachineMode}, gpr::GPR, pc::PC};

#[derive(Debug, Clone)]
pub struct MachineModel {
    pub gpr: GPR,
    pub csr: CSR,
    pub pc: PC,
    pub mode: Cell<MachineMode>,
}

const MISA64: u64
    = base_misa(BaseISA::RV64I)
    | misa_flag(b'm')
    // | misa_flag(b'a')
    | misa_flag(b'c')
    ;

impl MachineModel {
    #[inline]
    pub fn new(hart_id: u64) -> MachineModel {
        MachineModel {
            gpr: GPR::new(),
            csr: CSR::new(MISA64, hart_id),
            pc: PC::new(0),
            mode: Cell::new(MachineMode::Machine),
        }
    }
}

impl RegInfo for MachineModel {
    #[inline]
    fn get_reg_value(&self, reg: &str) -> Option<u64> {
        match reg {
            "pc" => Some(self.pc.read()),
            _ => {
                if let Ok(x) = reg.parse::<usize>() {
                    return if x < 32 {
                        Some(self.gpr.read(x))
                    } else {
                        None
                    };
                }
                let map = &REG_MAP;
                let (rt, r) = map.get(reg)?;
                if rt == &RegType::Gpr {
                    Some(self.gpr.read(*r))
                } else if rt == &RegType::Csr {
                    Some(self.csr.read(*r))
                } else {
                    unimplemented!()
                }
            },
        }
    }
}


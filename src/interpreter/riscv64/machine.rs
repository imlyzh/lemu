use crate::abstract_machine::RegInfo;

use super::reg::{REG_MAP, RegType, csr::CSR, gpr::GPR, pc::PC};

#[derive(Debug, Clone)]
pub struct MachineModel {
    pub gpr: GPR,
    pub csr: CSR,
    pub pc: PC,
}

impl MachineModel {
    #[inline]
    pub fn new() -> MachineModel {
        MachineModel {
            gpr: GPR::new(),
            csr: CSR::new(),
            pc: PC::new(0),
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
                if rt == &RegType::GPR {
                    Some(self.gpr.read(*r))
                } else if rt == &RegType::CSR {
                    Some(self.csr.read(*r))
                } else {
                    unimplemented!()
                }
            },
        }
    }
}


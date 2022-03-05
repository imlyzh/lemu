use crate::{
    abstract_machine::Execable,
    utils::field_range_into_u8,
    memory
};

use super::{
    machine::MachineModel,
    inst_type::*
};



impl MachineModel {
    fn invalid_inst(&self) {
        todo!()
    }
}

impl MachineModel {

    /// lui
    #[inline(always)]
    fn inst_0110111(&self, inst: &UType, memory: &memory::Memory) {

    }

    /// auipc
    #[inline(always)]
    fn inst_0010111(&self, inst: &UType, memory: &memory::Memory) {

    }

    /// jal
    #[inline(always)]
    fn inst_1101111(&self, inst: &JType, memory: &memory::Memory) {

    }

    /// jalr
    #[inline(always)]
    fn inst_1100111(&self, inst: &IType, memory: &memory::Memory) {

    }

    /// branch
    #[inline(always)]
    fn inst_1100011(&self, inst: &BType, memory: &memory::Memory) {

    }

    /// load
    #[inline(always)]
    fn inst_0000011(&self, inst: &IType, memory: &memory::Memory) {
        let addr = self.read_gpr(inst.rs1() as usize);
        let r = match inst.funct3() {
            0b000 => {
                let r = memory.read_u8(addr as usize);
                if r.is_none() {
                    // todo: memory read error
                };
                r.unwrap() as u64
            }
            0b001 => {
                let r = memory.read_u16(addr as usize);
                if r.is_none() {
                    // todo: memory read error
                };
                r.unwrap() as u64
            }
            0b010 => {
                let r = memory.read_u32(addr as usize);
                if r.is_none() {
                    // todo: memory read error
                };
                r.unwrap() as u64
            }
            _ => {
                self.invalid_inst();
                return;
            }
        };
    }

    /// store
    #[inline(always)]
    fn inst_0100011(&self, inst: &SType, memory: &memory::Memory) {
        let addr = self.read_gpr(inst.rs1() as usize);

    }

    /// op imm
    #[inline(always)]
    fn inst_0010011(&self, inst: &IType, memory: &memory::Memory) {
        let addr = self.read_gpr(inst.rs1() as usize);

    }

    /// op
    #[inline(always)]
    fn inst_0110011(&self, inst: &RType, memory: &memory::Memory) {
        let addr = self.read_gpr(inst.rs1() as usize);

    }

    /// fence
    #[inline(always)]
    fn inst_0001111(&self, inst: &IType, memory: &memory::Memory) {
        let addr = self.read_gpr(inst.rs1() as usize);

    }

    // privileged
    fn inst_1110011(&self, inst: &IType, memory: &memory::Memory) {
        let addr = self.read_gpr(inst.rs1() as usize);

    }
}

impl Execable for MachineModel {
    fn exec_once(&self, memory: &memory::Memory) {
        let pc = self.read_next();
        let code = memory.read_u32(pc as usize);
        if code.is_none() {
            // todo: error
            return;
        }
        let code = code.unwrap();

        match field_range_into_u8(code, 0, 7) {
            0b0110111 => self.inst_0110111(&UType::from_bytes(code.to_le_bytes()), memory),
            0b0010111 => self.inst_0010111(&UType::from_bytes(code.to_le_bytes()), memory),
            0b1101111 => self.inst_1101111(&JType::from_bytes(code.to_le_bytes()), memory),
            0b1100111 => self.inst_1100111(&IType::from_bytes(code.to_le_bytes()), memory),
            0b1100011 => self.inst_1100011(&BType::from_bytes(code.to_le_bytes()), memory),
            0b0000011 => self.inst_0000011(&IType::from_bytes(code.to_le_bytes()), memory),
            0b0100011 => self.inst_0100011(&SType::from_bytes(code.to_le_bytes()), memory),
            0b0010011 => self.inst_0010011(&IType::from_bytes(code.to_le_bytes()), memory),
            0b0110011 => self.inst_0110011(&RType::from_bytes(code.to_le_bytes()), memory),
            0b0001111 => self.inst_0001111(&IType::from_bytes(code.to_le_bytes()), memory),
            0b1110011 => self.inst_1110011(&IType::from_bytes(code.to_le_bytes()), memory),
            _ => {
                self.invalid_inst();
                return;
            }
        }
    }
}

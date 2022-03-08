use std::ops::BitAnd;

use crate::{
    abstract_machine::Execable,
    utils::{field_range_into_u8, field_range_into_u16},
    memory::{self, Memory}
};

use super::{
    machine::MachineModel,
    inst_type::*
};



impl MachineModel {
    fn invalid_inst(&self, pc: u64) {
        panic!("invalid opcode at pc={}", pc);
    }
}

impl MachineModel {

    /// lui
    #[inline(always)]
    fn inst_0110111(&self, inst: &UType, _: &memory::Memory) {
        let imm = inst.imm().overflowing_shl(12).0 as i32 as i64 as u64;
        self.store_gpr(inst.rd().into(), imm);
        self.set_pc(self.read_pc() + 4);
    }

    /// auipc
    #[inline(always)]
    fn inst_0010111(&self, inst: &UType, _: &memory::Memory) {
        let imm = inst.imm().overflowing_shl(12).0 as i32 as i64 as u64;
        self.store_gpr(inst.rd().into(), self.read_pc() + imm);
        self.set_pc(self.read_pc() + 4);
    }

    /// jal
    #[inline(always)]
    fn inst_1101111(&self, inst: &JType, _: &memory::Memory) {
        let pc = self.read_pc();
        self.store_gpr(inst.rd().into(), pc + 4);
        let next_pc = pc as i64 + inst.get_imm() as i64;
        self.set_pc(next_pc as u64);
    }

    /// jalr
    #[inline(always)]
    fn inst_1100111(&self, inst: &IType, _: &memory::Memory) {
        let pc = self.read_pc();
        self.store_gpr(inst.rd().into(), pc + 4);
        let next_pc =
            (self.read_gpr(inst.rs1().into()) as i64) + inst.sext_offset() as i64;
        self.set_pc(next_pc as u64);
    }

    /// branch
    #[inline(always)]
    fn inst_1100011(&self, inst: &BType, _: &memory::Memory) {
        let rs1 = self.read_gpr(inst.rs1().into());
        let rs2 = self.read_gpr(inst.rs2().into());
        let cond = match inst.funct3() {
            0b000 => rs1 == rs2,    // beq
            0b001 => rs1 != rs2,    // bne
            0b111 => rs1 >= rs2,    // bgeu
            0b110 => rs1 < rs2,     // bltu
            0b101 => (rs1 as i64) >= (rs2 as i64),  // bge
            0b100 => (rs1 as i64) < (rs2 as i64),   // blt
            _ => {
                self.invalid_inst(self.read_pc());
                return;
            },
        };
        if cond {
            let next_pc = self.read_pc() as i64 + inst.sext_offset() as i64;
            self.set_pc(next_pc as u64);
        } else {
            self.set_pc(self.read_pc() + 4);
        }
    }

    /// load
    #[inline(always)]
    fn inst_0000011(&self, inst: &IType, memory: &memory::Memory) {
        let addr = self.read_gpr(inst.rs1().into());
        let offset = inst.sext_imm() as i64;
        let addr = addr as i64 + offset;
        let addr = addr as usize;
        let r = match inst.funct3() {
            0b000 => memory.read_u8(addr).map(|x| x as i8 as i64 as u64),    // lb
            0b001 => memory.read_u16(addr).map(|x| x as i16 as i64 as u64), // lh
            0b010 => memory.read_u32(addr).map(|x| x as i32 as i64 as u64), // lw
            0b011 => memory.read_u64(addr), // ld
            0b100 => memory.read_u8(addr).map(|x| x as u64),    // lbu
            0b101 => memory.read_u16(addr).map(|x| x as u64),   // lhu
            0b110 => memory.read_u32(addr).map(|x| x as u64),   // lwu
            _ => {
                self.invalid_inst(self.read_pc());
                return;
            }
        };
        if r.is_none() {
            // self.invalid_memory_io();
            // return;
            todo!();
        }
        let r = r.unwrap();
        self.store_gpr(inst.rd().into(), r);
        self.set_pc(self.read_pc() + 4);
    }

    /// store
    #[inline(always)]
    fn inst_0100011(&self, inst: &SType, memory: &memory::Memory) {
        let addr = self.read_gpr(inst.rs1() as usize);
        let sext_offset = inst.sext_imm();
        let addr = addr as i64 + sext_offset as i64;
        let addr = addr as u64 as usize;
        match inst.funct3() {
            0b000 => memory.write_u8(addr, inst.rs2() as u8),   // sb
            0b001 => memory.write_u16(addr, inst.rs2() as u16), // sh
            0b010 => memory.write_u32(addr, inst.rs2() as u32), // sw
            0b011 => memory.write_u64(addr, inst.rs2() as u64), // sd
            _ => {
                self.invalid_inst(self.read_pc());
                return;
            }
        };
        self.set_pc(self.read_pc() + 4);
    }

    /// op imm
    #[inline(always)]
    fn inst_0010011(&self, inst: &IType, _: &memory::Memory) {
        let rs1 = self.read_gpr(inst.rs1().into());
        let sext_offset = inst.sext_imm();
        let value = match inst.funct3() {
            0b000 => (rs1 as i64 + sext_offset as i64) as u64,      // addi
            0b010 => ((rs1 as i64) < (sext_offset as i64)) as u64,  // slti
            0b011 => (rs1 < (sext_offset as i64 as u64)) as u64,    // sltiu
            0b100 => (rs1 as i64 ^ sext_offset as i64) as u64,      // xori
            0b110 => (rs1 as i64 | sext_offset as i64) as u64,      // ori
            0b111 => (rs1 as i64 & sext_offset as i64) as u64,      // andi
            0b001 => match field_range_into_u16(inst.imm().into(), 12, 5) {
                0b0000000 => ((rs1 as i64) << (sext_offset as i64)) as u64, // slli
                _ => {
                    self.invalid_inst(self.read_pc());
                    return;
                }
            },
            0b101 => match field_range_into_u16(inst.imm().into(), 12, 5) {
                0b0000000 => rs1 >> (sext_offset as u64), // srli
                0b0100000 => (rs1 as i64 >> sext_offset as i64) as u64, // srai
                _ => {
                    self.invalid_inst(self.read_pc());
                    return;
                }
            },
            _ => {
                self.invalid_inst(self.read_pc());
                return;
            }
        };
        self.store_gpr(inst.rd().into(), value);
        self.set_pc(self.read_pc() + 4);
    }

    /// op
    #[inline(always)]
    fn inst_0110011(&self, inst: &RType, _: &memory::Memory) {
        let rs1 = self.read_gpr(inst.rs1().into());
        let rs2 = self.read_gpr(inst.rs2().into());
        let value = match inst.funct3() {
            0b000 => match inst.funct7() {
                0b0000000 => rs1 + rs2,// add
                0b0100000 => rs1 - rs2,// sub
                _ => {
                    self.invalid_inst(self.read_pc());
                    return;
                }
            },
            0b001 => rs1.overflowing_shl(rs2.bitand(0b111111) as u32).0,// sll
            0b010 => ((rs1 as i64) < (rs2 as i64)) as u64,              // slt
            0b011 => (rs1 < rs2) as u64,    // sltu
            0b100 => rs1 ^ rs2,             // xor
            0b101 => match inst.funct3() {
                0b0000000 => rs1 >> rs2,    // srl
                0b0100000 => (rs1 as i64).overflowing_shr(rs2.bitand(0b111111) as u32).0 as u64, // sra
                _ => {
                    self.invalid_inst(self.read_pc());
                    return;
                }
            }
            0b110 => rs1 | rs2,             // or
            0b111 => rs1 & rs2,             // and
            _ => {
                self.invalid_inst(self.read_pc());
                return;
            }
        };
        self.store_gpr(inst.rd().into(), value);
        self.set_pc(self.read_pc() + 4);
    }

    /// fence
    #[inline(always)]
    fn inst_0001111(&self, inst: &IType, memory: &memory::Memory) {
        // let addr = self.read_gpr(inst.rs1() as usize);
        // nop
        self.set_pc(self.read_pc() + 4);
    }

    // privileged
    #[inline(always)]
    fn inst_1110011(&self, inst: &IType, memory: &memory::Memory) {
        let addr = self.read_gpr(inst.rs1() as usize);
        todo!();
        self.set_pc(self.read_pc() + 4);
    }
}

impl Execable for MachineModel {
    fn exec_once(&self, memory: &memory::Memory) {
        let pc = self.read_pc();
        let code = memory.read_u32(pc as usize);
        if code.is_none() {
            // todo: error
            return;
        }
        let code = code.unwrap();

        match field_range_into_u8(code, 6, 0) {
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
                self.invalid_inst(self.read_pc());
                return;
            }
        }
    }
}


#[test]
fn test_rv_eval() {
    let mm = MachineModel::new();
    // lui x1, 114514
    // addi x1, x1, 1919
    // add x1, x1, x0
    // add x1, x1, x1
    // sub x1, x1, x0
    // sub x1, x1, x1
    let inst_list = [
        469049527,
        2012250259,
        32947,
        1081523,
        1073774771,
        1074823347,
        ]
    .into_iter().flat_map(|x: u32| x.to_le_bytes()).collect();
    let mem = Memory::from(&inst_list);
    mm.exec_once(&mem);
    mm.exec_once(&mem);
    let value = (114514 << 12) + 1919;
    assert_eq!(mm.read_gpr(1), value);
    mm.exec_once(&mem);
    assert_eq!(mm.read_gpr(1), value);
    mm.exec_once(&mem);
    assert_eq!(mm.read_gpr(1), value+value);
    mm.exec_once(&mem);
    assert_eq!(mm.read_gpr(1), value+value);
    mm.exec_once(&mem);
    assert_eq!(mm.read_gpr(1), 0);

}
use std::ops::BitAnd;

use lyuu_commons::isa::riscv::{
    inst_binary::*,
    RiscV,
};
use lyuu_commons::disassembly::riscv::disassembly;

use crate::{
    abstract_machine::Execable,
    utils::{
        field_range_into_u8,
        field_range_into_u16,
    },
    device::MMIODevice
};

use super::{machine::MachineModel, irq::Exception};


macro_rules! gpr {
    ($this:ident, $offset:expr) => {
        $this.gpr.read($offset as usize)
    };
}

macro_rules! wgpr {
    ($this:ident, $offset:expr, $v:expr) => {
        $this.gpr.store($offset as usize, $v as u64)
    };
}

macro_rules! csr {
    ($this:ident, $offset:expr) => {
        $this.csr.read($offset as usize)
    };
}

macro_rules! wcsr {
    ($this:ident, $offset:expr, $v:expr) => {
        $this.csr.store($offset as usize, $v as u64)
    };
}

macro_rules! pc {
    ($this:ident) => {
        $this.pc.read()
    };
}

macro_rules! wpc {
    ($this:ident, $v:expr) => {
        $this.pc.store($v as u64)
    };
}

macro_rules! addpc {
    ($this:ident, $v:expr) => {
        wpc!($this, pc!($this) + $v as u64);
    };
}




impl MachineModel {

    /// lui
    #[inline]
    fn inst_0110111(&self, inst: &UType) {
        let imm = inst.imm().overflowing_shl(12).0 as i32 as i64 as u64;
        wgpr!(self, inst.rd(), imm);
        addpc!(self, 4);
    }

    /// auipc
    #[inline]
    fn inst_0010111(&self, inst: &UType) {
        let imm = inst.imm().overflowing_shl(12).0 as i32 as i64 as u64;
        wgpr!(self, inst.rd(), pc!(self) + imm);
        addpc!(self, 4);
    }

    /// jal
    #[inline]
    fn inst_1101111(&self, inst: &JType) {
        wgpr!(self, inst.rd(), pc!(self) + 4);
        let imm = inst.get_offset();
        let next_pc = pc!(self) as i64 + imm as i64;
        wpc!(self, next_pc);
    }

    /// jalr
    #[inline]
    fn inst_1100111(&self, inst: &IType) {
        wgpr!(self, inst.rd(), pc!(self) + 4);
        let next_pc = gpr!(self, inst.rs1()) as i64 + inst.sext_offset() as i64;
        wpc!(self, next_pc);
    }

    /// branch
    #[inline]
    fn inst_1100011(&self, inst: &BType) -> Result<(), Exception> {
        let rs1 = gpr!(self, inst.rs1());
        let rs2 = gpr!(self, inst.rs2());

        let cond = match inst.funct3() {
            0b000 => rs1 == rs2,    // beq
            0b001 => rs1 != rs2,    // bne
            0b111 => rs1 >= rs2,    // bgeu
            0b110 => rs1 < rs2,     // bltu
            0b101 => (rs1 as i64) >= (rs2 as i64),  // bge
            0b100 => (rs1 as i64) < (rs2 as i64),   // blt
            _ => return Err(Exception::IllegalInstruction),
        };
        if cond {
            let next_pc = pc!(self) as i64 + inst.sext_offset() as i64;
            wpc!(self, next_pc);
        } else {
            addpc!(self, 4);
        }
        Ok(())
    }

    /// load
    #[inline]
    fn inst_0000011(&self, inst: &IType, memory: &dyn MMIODevice) -> Result<(), Exception> {
        let addr = gpr!(self, inst.rs1());
        let offset = inst.sext_imm() as i64;
        let addr = addr as i64 + offset;
        let naddr = addr as usize;
        let r = match inst.funct3() {
            0b000 => memory.read_u8(naddr).map(|x| x as i8 as i64 as u64),    // lb
            0b001 => memory.read_u16(naddr).map(|x| x as i16 as i64 as u64), // lh
            0b010 => memory.read_u32(naddr).map(|x| x as i32 as i64 as u64), // lw
            0b011 => memory.read_u64(naddr), // ld
            0b100 => memory.read_u8(naddr).map(|x| x as u64),   // lbu
            0b101 => memory.read_u16(naddr).map(|x| x as u64),  // lhu
            0b110 => memory.read_u32(naddr).map(|x| x as u64),  // lwu
            _ => return Err(Exception::IllegalInstruction),
        };
        if r.is_none() {
            return Err(Exception::LoadAccessFault(addr as u64));
        }
        let r = r.unwrap();
        wgpr!(self, inst.rd(), r);
        addpc!(self, 4);
        Ok(())
    }

    /// store
    #[inline]
    fn inst_0100011(&self, inst: &SType, memory: &dyn MMIODevice) -> Result<(), Exception> {
        let addr = gpr!(self, inst.rs1());
        let sext_offset = inst.sext_imm();
        let addr = addr as i64 + sext_offset as i64;
        let addr = addr as u64 as usize;
        match inst.funct3() {
            0b000 => memory.write_u8(addr, inst.rs2() as u8),   // sb
            0b001 => memory.write_u16(addr, inst.rs2() as u16), // sh
            0b010 => memory.write_u32(addr, inst.rs2() as u32), // sw
            0b011 => memory.write_u64(addr, inst.rs2() as u64), // sd
            _ => return Err(Exception::IllegalInstruction),
        };
        addpc!(self, 4);
        Ok(())
    }

    /// op imm
    #[inline]
    fn inst_0010011(&self, inst: &IType) -> Result<(), Exception> {
        let rs1 = gpr!(self, inst.rs1());
        let sext_offset = inst.sext_imm();
        let value = match inst.funct3() {
            0b000 => (rs1 as i64 + sext_offset as i64) as u64,      // addi
            0b010 => ((rs1 as i64) < (sext_offset as i64)) as u64,  // slti
            0b011 => (rs1 < (sext_offset as i64 as u64)) as u64,    // sltiu
            0b100 => (rs1 as i64 ^ sext_offset as i64) as u64,      // xori
            0b110 => (rs1 as i64 | sext_offset as i64) as u64,      // ori
            0b111 => (rs1 as i64 & sext_offset as i64) as u64,      // andi
            0b001 => //((rs1 as i64) << (sext_offset as i64)) as u64, // slli
            // /*
            match field_range_into_u8(inst.imm().into(), 12, 6) {
                0b000000 => ((rs1 as i64) << (sext_offset as i64)) as u64, // slli
                _ => return Err(Exception::IllegalInstruction),
            },
            // */
            0b101 => match field_range_into_u8(inst.imm().into(), 12, 6) {
                0b000000 => rs1 >> (sext_offset as u64), // srli
                0b010000 => (rs1 as i64 >> sext_offset as i64) as u64, // srai
                _ =>  return Err(Exception::IllegalInstruction),
            },
            _ =>  return Err(Exception::IllegalInstruction),
        };
        wgpr!(self, inst.rd(), value);
        addpc!(self, 4);
        Ok(())
    }

    /// op imm word
    #[inline]
    fn inst_0011011(&self, inst: &IType) -> Result<(), Exception> {
        let rs1 = gpr!(self, inst.rs1());
        let sext_offset = inst.sext_imm();
        let value = match inst.funct3() {
            0b000 => (rs1 as i64 + sext_offset as i64) as u64 as u32 as i64,      // addiw
            0b001 => match field_range_into_u16(inst.imm().into(), 12, 5) {
                0b0000000 => ((rs1 as i64) << (sext_offset as i64)) as u64 as u32 as i64, // slliw
                _ => return Err(Exception::IllegalInstruction),
            },
            0b101 => match field_range_into_u16(inst.imm().into(), 12, 5) {
                0b0000000 => (rs1 >> (sext_offset as u64)) as u32 as i64, // srliw
                0b0100000 => (rs1 as i64 >> sext_offset as i64) as u64 as u32 as i64, // sraiw
                _ =>  return Err(Exception::IllegalInstruction),
            },
            _ =>  return Err(Exception::IllegalInstruction),
        };
        wgpr!(self, inst.rd(), value);
        addpc!(self, 4);
        Ok(())
    }

    /// op
    #[inline]
    fn inst_0110011(&self, inst: &RType) -> Result<(), Exception> {
        let rs1 = gpr!(self, inst.rs1());
        let rs2 = gpr!(self, inst.rs2());
        let value = match inst.funct3() {
            0b000 => match inst.funct7() {
                0b0000000 => rs1 + rs2,// add
                0b0100000 => rs1 - rs2,// sub
                _ =>  return Err(Exception::IllegalInstruction),
            },
            0b001 => rs1.overflowing_shl(rs2.bitand(0b111111) as u32).0,// sll
            0b010 => ((rs1 as i64) < (rs2 as i64)) as u64,              // slt
            0b011 => (rs1 < rs2) as u64,    // sltu
            0b100 => rs1 ^ rs2,             // xor
            0b101 => match inst.funct7() {
                0b0000000 => rs1 >> rs2,    // srl
                0b0100000 => (rs1 as i64).overflowing_shr(rs2.bitand(0b111111) as u32).0 as u64, // sra
                _ =>  return Err(Exception::IllegalInstruction),
            }
            0b110 => rs1 | rs2,             // or
            0b111 => rs1 & rs2,             // and
            _ => return Err(Exception::IllegalInstruction),
        };
        wgpr!(self, inst.rd(), value);
        addpc!(self, 4);
        Ok(())
    }

    /// op word
    #[inline]
    fn inst_0111011(&self, inst: &RType) -> Result<(), Exception> {
        let rs1 = gpr!(self, inst.rs1());
        let rs2 = gpr!(self, inst.rs2());
        let value = match inst.funct3() {
            0b000 => match inst.funct7() {
                0b0000000 => (rs1 + rs2) as u32 as i64,// addw
                0b0100000 => (rs1 - rs2) as u32 as i64,// subw
                _ =>  return Err(Exception::IllegalInstruction),
            },
            0b001 => rs1.overflowing_shl(rs2.bitand(0b111111) as u32).0 as u32 as i64,// sllw
            0b101 => match inst.funct7() {
                0b0000000 => (rs1 >> rs2) as u32 as i64,    // srlw
                0b0100000 => (rs1 as i64).overflowing_shr(rs2.bitand(0b111111) as u32).0 as u64 as u32 as i64, // sraw
                _ =>  return Err(Exception::IllegalInstruction),
            }
            _ => return Err(Exception::IllegalInstruction),
        };
        wgpr!(self, inst.rd(), value);
        addpc!(self, 4);
        Ok(())
    }

    /// fence
    #[inline(always)]
    fn inst_0001111(&self, _inst: &IType) {
        // nop
        addpc!(self, 4);
    }

    // privileged
    #[inline]
    fn inst_1110011(&self, inst: &IType, _memory: &dyn MMIODevice) -> Result<(), Exception> {
        // let rd = self.gpr.read(inst.rd() as usize);
        // let zimm = inst.rs1();
        match inst.funct3() {
            0b000 => match inst.imm() {
                0b0 => self.ecall(),
                0b1 => self.ebreak(),
                _ => return Err(Exception::IllegalInstruction),
            },
            0b001 => {
                let t = csr!(self, inst.csr());
                wcsr!(self, inst.csr(), gpr!(self, inst.rs1()));
                wgpr!(self, inst.rd(), t)
            },   // csrrw
            0b010 => {
                let t = csr!(self, inst.csr());
                wcsr!(self, inst.csr(), t|gpr!(self, inst.rs1()));
                wgpr!(self, inst.rd(), t)
            },   // csrrs
            0b011 => {
                let t = csr!(self, inst.csr());
                wcsr!(self, inst.csr(), t&!gpr!(self, inst.rs1()));
                wgpr!(self, inst.rd(), t)
            },   // csrrc
            0b101 => {
                wgpr!(self, inst.rd(), csr!(self, inst.csr()));
                let zimm = inst.rs1() as u64;
                wcsr!(self, inst.csr() as usize, zimm);
            },   // csrrwi
            0b110 => {
                let t = csr!(self, inst.csr());
                let zimm = inst.rs1() as u64;
                wcsr!(self, inst.csr(), t|zimm);
                wgpr!(self, inst.rd(), t);
            },   // csrrsi
            0b111 => {
                let t = csr!(self, inst.csr());
                let zimm = inst.rs1() as u64;
                wcsr!(self, inst.csr(), t&!zimm);
                wgpr!(self, inst.rd(), t);
            },   // csrrci
            _ => return Err(Exception::IllegalInstruction),
        }
        addpc!(self, 4);
        Ok(())
    }
}

impl Execable<Exception> for MachineModel {
    fn exec_once(&self, memory: &dyn MMIODevice) -> Result<(), Exception> {
        let pc = self.pc.read();
        let code = memory.read_u32(pc as usize);
        if code.is_none() {
            return Err(Exception::LoadAccessFault(pc));
        }
        let code = code.unwrap();
        println!("[lemu:itrace]  0x{:016x}:    {}\t{}",
            self.pc.read(),
            code.to_le_bytes().into_iter().map(|x| format!("{:02x}", x)).collect::<Vec<_>>().join(" "),
            disassembly(code).map_or("unimp".to_string(), |x| x.0.to_string()));

        match field_range_into_u8(code, 6, 0) {
            0b0110111 => self.inst_0110111(&UType::from_bytes(code.to_le_bytes())),
            0b0010111 => self.inst_0010111(&UType::from_bytes(code.to_le_bytes())),
            0b1101111 => self.inst_1101111(&JType::from_bytes(code.to_le_bytes())),
            0b1100111 => self.inst_1100111(&IType::from_bytes(code.to_le_bytes())),
            0b1100011 => self.inst_1100011(&BType::from_bytes(code.to_le_bytes()))?,
            0b0000011 => self.inst_0000011(&IType::from_bytes(code.to_le_bytes()), memory)?,
            0b0100011 => self.inst_0100011(&SType::from_bytes(code.to_le_bytes()), memory)?,
            0b0010011 => self.inst_0010011(&IType::from_bytes(code.to_le_bytes()))?,
            0b0011011 => self.inst_0011011(&IType::from_bytes(code.to_le_bytes()))?,
            0b0110011 => self.inst_0110011(&RType::from_bytes(code.to_le_bytes()))?,
            0b0111011 => self.inst_0111011(&RType::from_bytes(code.to_le_bytes()))?,
            0b0001111 => self.inst_0001111(&IType::from_bytes(code.to_le_bytes())),
            0b1110011 => self.inst_1110011(&IType::from_bytes(code.to_le_bytes()), memory)?,
            _ => return Err(Exception::IllegalInstruction),
        };
        Ok(())
    }
}

impl MachineModel {
    fn exec_inst(&self, inst: RiscV, _memory: &dyn MMIODevice) -> Result<(), Exception> {
        match inst {
            RiscV::Lui(_, _) => todo!(),
            RiscV::Auipc(_, _) => todo!(),
            RiscV::Jal(_, _) => todo!(),
            RiscV::Jalr(_, _, _) => todo!(),
            RiscV::Branch(_, _, _, _) => todo!(),
            RiscV::Load(_, _, _, _) => todo!(),
            RiscV::Store(_, _, _, _) => todo!(),
            RiscV::OpI(_, _, _, _) => todo!(),
            RiscV::Op(_, _, _, _) => todo!(),
            RiscV::Fence(_, _, _) => todo!(),
            RiscV::EOp(_) => todo!(),
            RiscV::CsrOp(_, _, _, _) => todo!(),
            RiscV::CsrOpI(_, _, _, _) => todo!(),
            RiscV::OpIW(_, _, _, _) => todo!(),
            RiscV::OpW(_, _, _, _) => todo!(),
        };
        // Ok(())
    }
}
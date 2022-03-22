
use lyuu_commons::isa::riscv::{
    *,
    inst_binary::*,
};


use crate::{
    utils::{
        field_range_into_u8,
        field_range_into_u16,
    },
};

/// lui
#[inline]
fn inst_0110111(inst: &UType) -> RiscV {
    let imm = inst.imm().overflowing_shl(12).0 as u32;
    let rd = inst.rd();
    RiscV::Lui(Reg(rd), imm)
}

/// auipc
#[inline]
fn inst_0010111(inst: &UType) -> RiscV {
    let imm = inst.imm().overflowing_shl(12).0 as u32;
    let rd = inst.rd();
    RiscV::Auipc(Reg(rd), imm)
}

/// jal
#[inline]
fn inst_1101111(inst: &JType) -> RiscV {
    let imm = inst.get_offset();
    let rd = inst.rd();
    RiscV::Jal(Reg(rd), imm)
}

/// jalr
#[inline]
fn inst_1100111(inst: &IType) -> RiscV {
    let imm = inst.sext_offset();
    let rs1 = inst.rs1();
    let rd = inst.rd();
    RiscV::Jalr(Reg(rd), Reg(rs1), imm)
}

/// branch
#[inline]
fn inst_1100011(inst: &BType) -> Option<RiscV> {
    let imm = inst.sext_offset();
    let rs1 = inst.rs1();
    let rs2 = inst.rs2();
    let cond = match inst.funct3() {
        0b000 => BrType::Eq,    // beq
        0b001 => BrType::Ne,    // bne
        0b111 => BrType::Geu,   // bgeu
        0b110 => BrType::Ltu,   // bltu
        0b101 => BrType::Ge,    // bge
        0b100 => BrType::Lt,    // blt
        _ => {
            return None;
        },
    };
    Some(RiscV::Branch(cond, Reg(rs1), Reg(rs2), imm))
}

/// load
#[inline]
fn inst_0000011(inst: &IType) -> Option<RiscV> {
    let rs1 = inst.rs1();
    let rd = inst.rd();
    let offset = inst.sext_imm();
    let ldty = match inst.funct3() {
        0b000 => LoadType::Byte,    // lb
        0b001 => LoadType::Half,    // lh
        0b010 => LoadType::Word,    // lw
        0b011 => LoadType::Double,  // ld
        0b100 => LoadType::ByteU,   // lbu
        0b101 => LoadType::HalfU,   // lhu
        0b110 => LoadType::WordU,   // lwu
        _ => {
            return None;
        }
    };
    Some(RiscV::Load(ldty, Reg(rd), Reg(rs1), offset))
}

/// store
#[inline]
fn inst_0100011(inst: &SType) -> Option<RiscV> {
    let rs1 = inst.rs1();
    let rs2 = inst.rs2();
    let offset = inst.sext_imm();
    let stty = match inst.funct3() {
        0b000 => StoreType::Byte,    // sb
        0b001 => StoreType::Half,    // sh
        0b010 => StoreType::Word,    // sw
        0b011 => StoreType::Double,  // sd
        _ => {
            return None;
        }
    };
    Some(RiscV::Store(stty, Reg(rs1), Reg(rs2), offset))
}

/// op imm
#[inline]
fn inst_0010011(inst: &IType) -> Option<RiscV> {
    let rs1 = inst.rs1();
    let rd = inst.rd();
    let sext_offset = inst.sext_imm();
    let iopty = match inst.funct3() {
        0b000 => IOpType::Addi, // addi
        0b010 => IOpType::Slti, // slti
        0b011 => IOpType::Sltiu,// sltiu
        0b100 => IOpType::Xori, // xori
        0b110 => IOpType::Ori,  // ori
        0b111 => IOpType::Andi, // andi
        0b001 => match field_range_into_u16(inst.imm().into(), 12, 5) {
            0b0000000 => IOpType::Slli, // slli
            _ => {
                return None;
            }
        },
        0b101 => match field_range_into_u16(inst.imm().into(), 12, 5) {
            0b0000000 => IOpType::Srli, // srli
            0b0100000 => IOpType::Srai, // srai
            _ => {
                return None;
            }
        },
        _ => {
            return None;
        }
    };
    Some(RiscV::OpI(iopty, Reg(rd), Reg(rs1), sext_offset))
}

/// op
#[inline]
fn inst_0110011(inst: &RType) -> Option<RiscV> {
    let rs1 = inst.rs1();
    let rs2 = inst.rs2();
    let rd = inst.rd();
    let opty = match inst.funct3() {
        0b000 => match inst.funct7() {
            0b0000000 => OpType::Add,// add
            0b0100000 => OpType::Sub,// sub
            _ => {
                return None;
            }
        },
        0b001 => OpType::Sll,// sll
        0b010 => OpType::Slt,              // slt
        0b011 => OpType::Sltu,    // sltu
        0b100 => OpType::Xor,             // xor
        0b101 => match inst.funct3() {
            0b0000000 => OpType::Srl,    // srl
            0b0100000 => OpType::Sra, // sra
            _ => {
                return None;
            }
        }
        0b110 => OpType::Or,             // or
        0b111 => OpType::And,             // and
        _ => {
            return None;
        }
    };
    Some(RiscV::Op(opty, Reg(rd), Reg(rs1), Reg(rs2)))
}

/// fence
#[inline(always)]
fn inst_0001111(inst: &IType) -> Option<RiscV> {
    todo!()
    // Some(RiscV::Fence(, (), ()))
}

// privileged
#[inline]
fn inst_1110011(inst: &IType) -> Option<RiscV> {
    todo!()
}

fn disassembly(code: u32) -> Option<RiscV> {
    let r = match field_range_into_u8(code, 6, 0) {
        0b0110111 => inst_0110111(&UType::from_bytes(code.to_le_bytes())),
        0b0010111 => inst_0010111(&UType::from_bytes(code.to_le_bytes())),
        0b1101111 => inst_1101111(&JType::from_bytes(code.to_le_bytes())),
        0b1100111 => inst_1100111(&IType::from_bytes(code.to_le_bytes())),
        0b1100011 => inst_1100011(&BType::from_bytes(code.to_le_bytes()))?,
        0b0000011 => inst_0000011(&IType::from_bytes(code.to_le_bytes()))?,
        0b0100011 => inst_0100011(&SType::from_bytes(code.to_le_bytes()))?,
        0b0010011 => inst_0010011(&IType::from_bytes(code.to_le_bytes()))?,
        0b0110011 => inst_0110011(&RType::from_bytes(code.to_le_bytes()))?,
        0b0001111 => inst_0001111(&IType::from_bytes(code.to_le_bytes()))?,
        0b1110011 => inst_1110011(&IType::from_bytes(code.to_le_bytes()))?,
        _ => {
            return None;
        }
    };
    Some(r)
}

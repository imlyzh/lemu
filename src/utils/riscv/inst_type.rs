use std::ops::BitAnd;

use modular_bitfield::prelude::*;


#[bitfield(bits = 32)]
pub struct RType {
    pub opcode: B7,
    pub rd: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub rs2: B5,
    pub funct7: B7,
}

#[bitfield(bits = 32)]
pub struct IType {
    pub opcode: B7,
    pub rd: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub imm: B12,
}

impl IType {
    pub fn sext_offset(&self) -> i16 {
        let sign = self.imm() >> 11;
        let filling: u16 = if sign == 0 {
            0
        } else {
            0b1111 << 12
        };
        (filling | (self.imm().bitand(0b11111111111) << 1)) as i16
    }

    pub fn sext_imm(&self) -> i16 {
        let sign = self.imm() >> 11;
        let filling: i16 = if sign == 0 {
            0
        } else {
            0b11111 << 11
        };
        filling | self.imm().bitand(0b11111111111) as i16
    }
}

#[bitfield(bits = 32)]
pub struct SType {
    pub opcode: B7,
    pub imm4_0: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub rs2: B5,
    pub imm11_5: B7,
}

impl SType {
    pub fn sext_imm(&self) -> i16 {
        let filling: i16 = if self.imm11_5().overflowing_shr(6).0 == 0 {
            0
        } else {
            0b11111 << 11
        };
        let val = (self.imm11_5() as u16).bitand(0b111111).overflowing_shl(5).0 | self.imm4_0() as u16;
        filling | val as i16
    }
}

#[bitfield(bits = 32)]
pub struct BType {
    pub opcode: B7,
    pub imm11: B1,
    pub imm4_1: B4,
    pub funct3: B3,
    pub rs1: B5,
    pub rs2: B5,
    pub imm10_5: B6,
    pub imm12: B1,
}

impl BType {
    pub fn sext_offset(&self) -> i16 {
        let filling: u16 = if self.imm12() == 0 {
            0
        } else {
            0b1111 << 12
        };
        (
            filling                         |
            (self.imm11()   as u16) << 11   |
            (self.imm10_5() as u16) << 5    |
            (self.imm4_1()  as u16) << 1
        ) as i16
    }
}

#[bitfield(bits = 32)]
pub struct UType {
    pub opcode: B7,
    pub rd: B5,
    pub imm: B20,
}

#[bitfield(bits = 32)]
pub struct JType {
    pub opcode: B7,
    pub rd: B5,
    pub imm19_12: B8,
    pub imm11: B1,
    pub imm10_1: B10,
    pub imm20: B1,
}

impl JType {
    #[inline]
    pub fn get_offset(&self) -> i32 {
        let filling = if self.imm20() == 0 {
            0
        } else {
            0b111111111111 << 20
        };
        let imm19_12 = dbg!(self.imm19_12()) as u32;
        let imm11 = dbg!(self.imm11()) as u32;
        let imm10_1 = dbg!(self.imm10_1()) as u32;
        let r = (
            (imm10_1 << 1)  |
            (imm11 << 11)   |
            (imm19_12 << 12)|
            filling
        ) as i32;
        dbg!(r)

    }
}

#[test]
fn bit_field_test() {
    let inst: u32 = 0b00000000000000000000_00001_0110111;
    let inst0 = UType::from_bytes(inst.to_ne_bytes());
    assert_eq!(inst0.imm(), 0);
    assert_eq!(inst0.opcode(), 0b0110111);
    assert_eq!(inst0.rd(), 0b00001);
}

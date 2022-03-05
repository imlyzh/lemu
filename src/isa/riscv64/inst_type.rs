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

#[bitfield(bits = 32)]
pub struct SType {
    pub opcode: B7,
    pub imm4_0: B5,
    pub funct3: B3,
    pub rs1: B5,
    pub rs2: B5,
    pub imm: B7,
}

#[bitfield(bits = 32)]
pub struct BType {
    pub opcode: B7,
    pub imm11: B1,
    pub imm4_1: B4,
    pub funct3: B3,
    pub rs1: B5,
    pub rs2: B5,
    pub imm5_0: B6,
    pub imm12: B1,
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


#[test]
fn bit_field_test() {
    let inst: u32 = 0b00000000000000000000_00001_0110111;
    let inst0 = UType::from_bytes(inst.to_ne_bytes());
    assert_eq!(inst0.imm(), 0);
    assert_eq!(inst0.opcode(), 0b0110111);
    assert_eq!(inst0.rd(), 0b00001);
}

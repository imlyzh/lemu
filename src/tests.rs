
use lyuu_commons::isa::riscv::inst_binary::UType;

use crate::{disassembly::riscv::*, interpreter::riscv64::machine::MachineModel, memory::Memory, abstract_machine::Execable};

#[test]
#[cfg(debug_assertions)]
fn test_rv_eval() {
    let mm = MachineModel::new(0);
    // lui x1, 114514
    // addi x1, x1, 1919
    // add x1, x1, x0
    // add x1, x1, x1
    // sub x1, x1, x0
    // sub x1, x1, x1
    // jal x0, -6*4
    let inst_list: Vec<u8> = [
        469049527,
        2012250259,
        32947,
        1081523,
        1073774771,
        1074823347,
        4271894639,
        ]
    .into_iter().flat_map(|x: u32| x.to_le_bytes()).collect();
    let mem = Memory::from(inst_list.as_ref());
    mm.exec_once(&mem);
    assert_eq!(mm.gpr.read(1), (114514 << 12));
    mm.exec_once(&mem);
    let value = (114514 << 12) + 1919;
    assert_eq!(mm.gpr.read(1), value);
    mm.exec_once(&mem);
    assert_eq!(mm.gpr.read(1), value);
    mm.exec_once(&mem);
    assert_eq!(mm.gpr.read(1), value+value);
    mm.exec_once(&mem);
    assert_eq!(mm.gpr.read(1), value+value);
    mm.exec_once(&mem);
    assert_eq!(mm.gpr.read(1), 0);
    assert_eq!(mm.pc.read(), 24);
    mm.exec_once(&mem);
    assert_eq!(mm.pc.read(), 0);
    mm.exec_once(&mem);
    assert_eq!(mm.gpr.read(1), (114514 << 12));
}

#[test]
fn test_jalr() {
    let mm = MachineModel::new(0);
    // jalr x0, x0, 0
    let inst_list: Vec<u8> = [
        103
        ]
    .into_iter().flat_map(|x: u32| x.to_le_bytes()).collect();
    let mem = Memory::from(inst_list.as_ref());
    mm.exec_once(&mem);
    assert_eq!(mm.pc.read(), 0);
}

#[test]
fn test_loop() {
    let mm = MachineModel::new(0);
    let inst_list: Vec<u8> = [
        0x00006f,
        ]
    .into_iter().flat_map(|x: u32| x.to_le_bytes()).collect();
    let mem = Memory::from(inst_list.as_ref());
    mm.exec_once(&mem);
    assert_eq!(mm.pc.read(), 0);
    mm.exec_once(&mem);
    assert_eq!(mm.pc.read(), 0);
}

#[test]
fn test_br() {
    let mm = MachineModel::new(0);
    // jalr x0, x0, 0
    let inst_list: Vec<u8> = [
        0x263,
        0x1063,
        0xfe000ce3,
        ]
    .into_iter().flat_map(|x: u32| x.to_le_bytes()).collect();
    let mem = Memory::from(inst_list.as_ref());
    mm.exec_once(&mem);
    assert_eq!(mm.pc.read(), 4);
    mm.exec_once(&mem);
    assert_eq!(mm.pc.read(), 8);
    mm.exec_once(&mem);
    assert_eq!(mm.pc.read(), 0);
}
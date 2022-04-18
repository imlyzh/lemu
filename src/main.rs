#![allow(dead_code)]
mod memory;
mod monitor;
mod device;
mod abstract_machine;
mod interpreter;
// mod disassembly;
mod utils;
#[cfg(test)]
mod tests;


use std::io::{stdout, Write};

// use clap::Parser;
// use disassembly::riscv::disassembly;

use crate::{
    abstract_machine::*,
    interpreter::riscv64::machine::MachineModel,
    device::{Device}, memory::Memory,
};

const BL: &[u8] = include_bytes!("../tests/bbl.bin");
// const BL: &[u8] = include_bytes!("../tests/rv64ui-p-addi.bin");

fn main() {
    println!("Welecome to lemu!");
    let mm = MachineModel::new(0);
    mm.pc.store(0x80000000);
    let mut mmio = Device::new();
    let bootloader = Memory::from(BL);
    mmio.add_device(0x80000000, Box::new(bootloader));
    let mem = Memory::new(128*1024*1024); // init 128Kb
    mmio.add_device(0x80020000, Box::new(mem));
    loop {
        mm.logged_process_exception(&mmio, mm.exec_catch_interrupt_loop(&mmio));
        // mm.logged_process_exception(&mmio, mm.exec_loop_otherwise_debugger_trap(&mmio));
        print!("Type enter continue: ");
        stdout().flush().unwrap();
        let mut _i = String::new();
        std::io::stdin().read_line(&mut _i).unwrap();
    }
}

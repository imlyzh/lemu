// #![feature(unchecked_math)]
mod memory;
mod monitor;
mod device;
mod abstract_machine;
mod interpreter;
mod disassembly;
mod utils;
#[cfg(test)]
mod tests;


use std::io::{stdout, Write};

use clap::Parser;

use crate::{
    abstract_machine::*,
    interpreter::riscv64::machine::MachineModel,
    device::{Device, MMIODevice}, memory::Memory,
};

fn main() {
    println!("Welcome to lemu!");
    let mm = MachineModel::new(0);
    mm.pc.store(0x80000000);
    let mut mmio = Device::new();
    let bootloader = Memory::from(include_bytes!("../tests/bbl.bin").as_ref());
    mmio.add_device(0x80000000, Box::new(bootloader));
    let mem = Memory::new(128*1024*1024); // init 128Kb
    mmio.add_device(0x80020000, Box::new(mem));
    loop {
        mm.logged_process_exception(&mmio, mm.exec_loop(&mmio));
        print!("Type enter continue: ");
        stdout().flush().unwrap();
        let mut _i = String::new();
        std::io::stdin().read_line(&mut _i).unwrap();
    }
}

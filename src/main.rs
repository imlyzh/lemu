use crate::{interpreter::riscv64::machine::MachineModel, abstract_machine::Execable};

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

fn main() {
    println!("Welcome to lemu!");
    let mm = MachineModel::new(0);
    mm.pc.store(0x80000000);
    // mm.exec_once(memory)
}

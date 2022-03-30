pub mod riscv;


use std::collections::BTreeMap;

use crate::abstract_machine::{Readable, Writeable, LengthInfo};


pub trait MMIODevice: LengthInfo + Readable + Writeable {}

pub struct Device {
    pub device_table: BTreeMap<usize, Box<dyn MMIODevice>>
}

impl Device {
    pub fn new() -> Device {
        Device {
            device_table: BTreeMap::new()
        }
    }

    pub fn add_device(&mut self, start_addr: usize, device: Box<dyn MMIODevice>) {
        self.device_table.insert(start_addr, device);
    }
}

impl Readable for Device {
    fn read_u8(&self, addr: usize) -> Option<u8> {
        for (start_addr, i) in self.device_table.range(0..addr+1) {
            if addr >= *start_addr && addr < *start_addr + i.get_length() {
                return i.read_u8(addr - start_addr);
            }
        }
        None
    }
    fn read_u16(&self, addr: usize) -> Option<u16> {
        for (start_addr, i) in self.device_table.range(0..addr+1) {
            if addr >= *start_addr && addr + 1 < *start_addr + i.get_length() {
                return i.read_u16(addr - start_addr);
            }
        }
        None
    }
    fn read_u32(&self, addr: usize) -> Option<u32> {
        for (start_addr, i) in self.device_table.range(0..addr+1) {
            if addr >= *start_addr && addr + 3 < *start_addr + i.get_length() {
                return i.read_u32(addr - start_addr);
            }
        }
        None
    }
    fn read_u64(&self, addr: usize) -> Option<u64> {
        for (start_addr, i) in self.device_table.range(0..addr+1) {
            if addr >= *start_addr && addr + 7 < *start_addr + i.get_length() {
                return i.read_u64(addr - start_addr);
            }
        }
        None
    }
}

impl Writeable for Device {
    fn write_u8(&self, addr: usize, value: u8) -> Option<()> {
        for (start_addr, i) in self.device_table.range(0..addr+1) {
            if addr >= *start_addr && addr < *start_addr + i.get_length() {
                return i.write_u8(addr - start_addr, value);
            }
        }
        None
    }

    fn write_u16(&self, addr: usize, value: u16) -> Option<()> {
        for (start_addr, i) in self.device_table.range(0..addr+1) {
            if addr >= *start_addr && addr + 1 < *start_addr + i.get_length() {
                return i.write_u16(addr - start_addr, value);
            }
        }
        None
    }

    fn write_u32(&self, addr: usize, value: u32) -> Option<()> {
        for (start_addr, i) in self.device_table.range(0..addr+1) {
            if addr >= *start_addr && addr + 3 < *start_addr + i.get_length() {
                return i.write_u32(addr - start_addr, value);
            }
        }
        None
    }

    fn write_u64(&self, addr: usize, value: u64) -> Option<()> {
        for (start_addr, i) in self.device_table.range(0..addr+1) {
            if addr >= *start_addr && addr + 7 < *start_addr + i.get_length() {
                return i.write_u64(addr - start_addr, value);
            }
        }
        None
    }
}

impl LengthInfo for Device {
    fn get_length(&self) -> usize {
        self.device_table.iter().map(|(_, d)| d.get_length()).sum()
    }
}

impl MMIODevice for Device {}
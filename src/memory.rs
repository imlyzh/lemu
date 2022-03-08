use std::cell::{Cell, RefCell};



pub struct Memory {
    mem: RefCell<Vec<u8>>,
}

impl Memory {
    #[inline(always)]
    pub fn new(limit: usize) -> Memory {
        assert!(limit > 0);
        let mem = Memory {
            mem: RefCell::new(Vec::new()),
        };
        mem.mem.borrow_mut().resize(limit, 0);
        mem
    }

    #[inline(always)]
    pub fn from(i: &Vec<u8>) -> Memory {
        let mem = Memory {
            mem: RefCell::new(i.clone()),
        };
        mem
    }

    #[inline(always)]
    pub fn read_u8(&self, addr: usize) -> Option<u8> {
        self.mem.borrow().get(addr).cloned()
    }

    #[inline(always)]
    pub fn read_u16(&self, addr: usize) -> Option<u16> {
        let l = self.read_u8(addr+0)? as u16;
        let h = self.read_u8(addr+1)? as u16;
        Some((h << 8) | l)
    }

    #[inline(always)]
    pub fn read_u32(&self, addr: usize) -> Option<u32> {
        let l = self.read_u16(addr+0)? as u32;
        let h = self.read_u16(addr+2)? as u32;
        Some((h << 16) | l)
    }

    #[inline(always)]
    pub fn read_u64(&self, addr: usize) -> Option<u64> {
        let l = self.read_u32(addr+0)? as u64;
        let h = self.read_u32(addr+4)? as u64;
        Some((h << 32) | l)
    }

    #[inline(always)]
    pub fn write_u8(&self, addr: usize, value: u8) -> Option<()> {
        *self.mem.borrow_mut().get_mut(addr)? = value;
        Some(())
    }

    #[inline(always)]
    pub fn write_u16(&self, addr: usize, value: u16) -> Option<()> {
        self.write_u8(addr+0, value as u8)?;
        self.write_u8(addr+1, (value >> 8) as u8)?;
        Some(())
    }

    #[inline(always)]
    pub fn write_u32(&self, addr: usize, value: u32) -> Option<()> {
        self.write_u16(addr+0, value as u16)?;
        self.write_u16(addr+2, (value >> 16) as u16)?;
        Some(())
    }

    #[inline(always)]
    pub fn write_u64(&self, addr: usize, value: u64) -> Option<()> {
        self.write_u32(addr+0, value as u32)?;
        self.write_u32(addr+4, (value >> 32) as u32)?;
        Some(())
    }
}
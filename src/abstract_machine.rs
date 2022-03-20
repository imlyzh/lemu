use crate::memory;


pub trait RegInfo {
    fn get_reg_value(&self, i: &str) -> Option<u64>;
}

pub trait LengthInfo {
    fn get_length(&self) -> usize;
}

pub trait Readable {

    #[inline]
    fn read_u8(&self, addr: usize) -> Option<u8> {
        None
    }

    #[inline]
    fn read_u16(&self, addr: usize) -> Option<u16> {
        let l = self.read_u8(addr+0)? as u16;
        let h = self.read_u8(addr+1)? as u16;
        Some((h << 8) | l)
    }

    #[inline]
    fn read_u32(&self, addr: usize) -> Option<u32> {
        let l = self.read_u16(addr+0)? as u32;
        let h = self.read_u16(addr+2)? as u32;
        Some((h << 16) | l)
    }

    #[inline]
    fn read_u64(&self, addr: usize) -> Option<u64> {
        let l = self.read_u32(addr+0)? as u64;
        let h = self.read_u32(addr+4)? as u64;
        Some((h << 32) | l)
    }
}

pub trait Writeable {
    #[inline]
    fn write_u8(&self, addr: usize, value: u8) -> Option<()> {
        None
    }

    #[inline]
    fn write_u16(&self, addr: usize, value: u16) -> Option<()> {
        self.write_u8(addr+0, value as u8)?;
        self.write_u8(addr+1, (value >> 8) as u8)?;
        Some(())
    }

    #[inline]
    fn write_u32(&self, addr: usize, value: u32) -> Option<()> {
        self.write_u16(addr+0, value as u16)?;
        self.write_u16(addr+2, (value >> 16) as u16)?;
        Some(())
    }

    #[inline]
    fn write_u64(&self, addr: usize, value: u64) -> Option<()> {
        self.write_u32(addr+0, value as u32)?;
        self.write_u32(addr+4, (value >> 32) as u32)?;
        Some(())
    }
}

pub trait Execable {
    fn exec_once(&self, memory: &memory::Memory);

    fn exec_loop(&self, memory: &memory::Memory) {
        loop {
            self.exec_once(memory);
        }
    }

    fn setp_num(&self, memory: &memory::Memory, num: usize) {
        for _ in 0..num {
            self.exec_once(memory);
        }
    }
}

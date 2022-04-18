use crate::device::MMIODevice;


pub trait RegInfo {
    fn get_reg_value(&self, i: &str) -> Option<u64>;
}

pub trait LengthInfo {
    fn get_length(&self) -> usize;
}

pub trait Readable {

    #[inline]
    fn read_u8(&self, _addr: usize) -> Option<u8> {
        None
    }

    #[inline]
    fn read_u16(&self, addr: usize) -> Option<u16> {
        let l = self.read_u8(addr)? as u16;
        let h = self.read_u8(addr+1)? as u16;
        Some((h << 8) | l)
    }

    #[inline]
    fn read_u32(&self, addr: usize) -> Option<u32> {
        let l = self.read_u16(addr)? as u32;
        let h = self.read_u16(addr+2)? as u32;
        Some((h << 16) | l)
    }

    #[inline]
    fn read_u64(&self, addr: usize) -> Option<u64> {
        let l = self.read_u32(addr)? as u64;
        let h = self.read_u32(addr+4)? as u64;
        Some((h << 32) | l)
    }

    #[inline]
    unsafe fn unchecked_read_u8(&self, addr: usize) -> u8 {
        self.read_u8(addr).unwrap()
    }

    #[inline]
    unsafe fn unchecked_read_u16(&self, addr: usize) -> u16 {
        let l = self.unchecked_read_u8(addr) as u16;
        let h = self.unchecked_read_u8(addr+1) as u16;
        (h << 8) | l
    }

    #[inline]
    unsafe fn unchecked_read_u32(&self, addr: usize) -> u32 {
        let l = self.unchecked_read_u16(addr) as u32;
        let h = self.unchecked_read_u16(addr+2) as u32;
        (h << 16) | l
    }

    #[inline]
    unsafe fn unchecked_read_u64(&self, addr: usize) -> u64 {
        let l = self.unchecked_read_u32(addr) as u64;
        let h = self.unchecked_read_u32(addr+4) as u64;
        (h << 32) | l
    }
}

pub trait Writeable {
    fn write_u8(&self, _addr: usize, _value: u8) -> Option<()> {
        None
    }

    #[inline]
    fn write_u16(&self, addr: usize, value: u16) -> Option<()> {
        self.write_u8(addr, value as u8)?;
        self.write_u8(addr+1, (value >> 8) as u8)?;
        Some(())
    }

    #[inline]
    fn write_u32(&self, addr: usize, value: u32) -> Option<()> {
        self.write_u16(addr, value as u16)?;
        self.write_u16(addr+2, (value >> 16) as u16)?;
        Some(())
    }

    #[inline]
    fn write_u64(&self, addr: usize, value: u64) -> Option<()> {
        self.write_u32(addr, value as u32)?;
        self.write_u32(addr+4, (value >> 32) as u32)?;
        Some(())
    }

    #[inline]
    unsafe fn unchecked_write_u8(&self, addr: usize, value: u8) {
        self.write_u8(addr, value).unwrap()
    }

    #[inline]
    unsafe fn unchecked_write_u16(&self, addr: usize, value: u16) {
        self.write_u8(addr, value as u8);
        self.write_u8(addr+1, (value >> 8) as u8);
    }

    #[inline]
    unsafe fn unchecked_write_u32(&self, addr: usize, value: u32) {
        self.write_u16(addr, value as u16);
        self.write_u16(addr+2, (value >> 16) as u16);
    }

    #[inline]
    unsafe fn unchecked_write_u64(&self, addr: usize, value: u64) {
        self.write_u32(addr, value as u32);
        self.write_u32(addr+4, (value >> 32) as u32);
    }
}


pub trait ExceptionAttr {
    fn is_debugger_trap(&self) -> bool;
}

pub trait Execable<E: ExceptionAttr + Clone>: ExceptionProcessable<E> {
    fn exec_once(&self, memory: &dyn MMIODevice) -> Result<(), E>;

    fn exec_catch_interrupt_loop(&self, memory: &dyn MMIODevice) -> Result<(), E> {
        loop {
            self.exec_once(memory)?;
        }
    }

    fn exec_loop_otherwise_debugger_trap(&self, memory: &dyn MMIODevice) -> Result<(), E> {
        loop {
            let r = self.exec_once(memory);
            self.process_exception(r.clone());
            if let Err(e) = r {
                if e.is_debugger_trap() {
                    return Err(e);
                }
            }
        }
    }

    fn setp_num(&self, memory: &dyn MMIODevice, num: usize) -> Result<(), E> {
        for _ in 0..num {
            self.exec_once(memory)?;
        }
        Ok(())
    }
}

pub trait ExceptionProcessable<E> {
    fn process_exception(&self, _e: Result<(), E>) {}
    fn exception_log(&self, _memory: &dyn MMIODevice, e: Result<(), E>) -> Result<(), E> { e }
    fn logged_process_exception(&self, memory: &dyn MMIODevice, e: Result<(), E>) {
        self.process_exception(self.exception_log(memory, e));
    }
}
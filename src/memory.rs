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
    pub fn read_mem(&self, addr: usize) -> Option<u8> {
        self.mem.borrow().get(addr).cloned()
    }

    #[inline(always)]
    pub fn write_mem(&self, addr: usize, value: u8) -> Option<()> {
        *self.mem.borrow_mut().get_mut(addr)? = value;
        Some(())
    }
}
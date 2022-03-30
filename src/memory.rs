use std::cell::{
    // Cell,
    RefCell
};

use crate::{abstract_machine::{LengthInfo, Readable, Writeable}, device::MMIODevice};



pub struct Memory {
    mem: RefCell<Vec<u8>>,
}

impl Memory {
    #[inline]
    pub fn new(limit: usize) -> Memory {
        assert!(limit > 0);
        let mem = Memory {
            mem: RefCell::new(Vec::new()),
        };
        mem.mem.borrow_mut().resize(limit, 0);
        mem
    }
}

impl From<&[u8]> for Memory {
    #[inline(always)]
    fn from(i: &[u8]) -> Self {
        let mem = Memory {
            mem: RefCell::new(i.to_vec()),
        };
        mem
    }
}

impl LengthInfo for Memory {
    #[inline]
    fn get_length(&self) -> usize {
        self.mem.borrow().len()
    }
}

impl Readable for Memory {
    fn read_u8(&self, addr: usize) -> Option<u8> {
        self.mem.borrow().get(addr).cloned()
    }
}

impl Writeable for Memory {
    fn write_u8(&self, addr: usize, value: u8) -> Option<()> {
        *self.mem.borrow_mut().get_mut(addr)? = value;
        Some(())
    }
}

impl MMIODevice for Memory {}

#[test]
fn demo() {
    let a = 4 as u64;
    let b = (a as i64 + (-4)) as u64;
    assert_eq!(b, 0);
}
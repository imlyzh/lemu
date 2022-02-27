use crate::memory;


pub trait RegInfo {
    fn get_reg_value(&self, i: &str) -> Option<u64>;
}

pub trait Execable {
    fn exec_once(&self, memory: &memory::Memory);

    fn exec_loop(&self, memory: &memory::Memory) {
        loop {
            self.exec_once(memory);
        }
    }

    fn exec_loop_num(&self, memory: &memory::Memory, num: usize) {
        for _ in 0..num {
            self.exec_once(memory);
        }
    }
}

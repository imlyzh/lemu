use std::mem::size_of;


pub struct BitField<const LEFT: usize, const RIGHT: usize> {
    pub value: usize,
}

impl<const LEFT: usize, const RIGHT: usize> BitField<LEFT, RIGHT> {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn get(&self) -> usize {
        let lshift = size_of::<usize>()*8 - LEFT - 1;
        let rshift = lshift + RIGHT;
        unsafe {
            self.value.unchecked_shl(lshift).unchecked_shr(rshift)
        }
    }
}

#[test]
fn bitfield_test() {
    let bf = BitField::<5, 0>::new(0b1010101010101010);
    assert_eq!(bf.get(), 0b101010);
    let bf = BitField::<10, 6>::new(0b1010111111101010);
    assert_eq!(bf.get(), 0b11111);
}
pub mod riscv;

use std::mem::size_of;


// Co-authored-by: Chuigda WhiteGive <icey@icey.tech>
macro_rules! make_get_field_range {
    ($name:ident, $t:ty) => {
        #[inline]
        pub fn $name(i: u32, left: usize, right: usize) -> $t {
            let lshift: usize = u32::BITS as usize - left - 1;
            let rshift: usize = lshift + right;
            i.overflowing_shl(lshift as u32).0.overflowing_shr(rshift as u32).0 as $t
        }
    }
}

make_get_field_range!(field_range_into_u8, u8);
make_get_field_range!(field_range_into_u16, u16);
make_get_field_range!(field_range_into_u32, u32);

/*
#[inline]
pub fn get_field_range(i: u32, left: usize, right: usize) -> u32 {
    let lshift = size_of::<usize>()*8 - left - 1;
    let rshift = lshift + right;
    i.overflowing_shl(lshift as u32).0.overflowing_shr(rshift as u32).0.into()
}
*/

#[test]
fn bitfield_range_test() {
    let bf = field_range_into_u8(0b1010101010101010, 5, 0);
    assert_eq!(bf, 0b101010);
    let bf = field_range_into_u8(0b1010111111101010, 10, 6);
    assert_eq!(bf, 0b11111);
}

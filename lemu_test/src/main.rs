#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::{
    panic::PanicInfo,
    arch::{global_asm, asm},
    intrinsics::volatile_load,
};
global_asm!(include_str!("entry.asm"));


// static mut DATAS: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];


#[no_mangle]
#[inline(never)]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 6), 4);
        asm!("ebreak");
    }
    loop {}
}

macro_rules! impl_v_op {
    ($name:ident, $op:tt) => {
        unsafe fn $name(i: u32, j: u32) -> u32 {
            volatile_load(&i) $op volatile_load(&j)
        }
    };
}

impl_v_op!(add, +);
impl_v_op!(sub, -);
impl_v_op!(mul, *);
impl_v_op!(div, /);


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

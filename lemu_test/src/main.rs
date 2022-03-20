#![no_std]
#![no_main]
#![feature(core_intrinsics)]
use core::{panic::PanicInfo, arch::asm};


static mut DATAS: [u8; 8] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        for i in core::intrinsics::volatile_load(&mut DATAS).iter_mut() {
            *i = 0xFF;
        }
        asm!("ebreak");
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
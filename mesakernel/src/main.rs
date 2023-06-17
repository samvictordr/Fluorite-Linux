// main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"MesaKernel - wip";
/// Mark C to use C calling convention
/// This function is the entry point!!
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// On-panic function
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {

    loop {}
}




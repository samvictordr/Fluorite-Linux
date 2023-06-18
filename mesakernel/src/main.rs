// main.rs

#![no_std]
#![no_main]
mod vga_buffer;

use core::panic::PanicInfo;

static GREET_FUNC: &[u8] = b"MesaKernel v0.1.0-rc1 - wip";
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
    //vga_buffer::test_output();
    println!("Hello From mesaKernel v.0.1.0-rc1 {}", "!");
    panic!("Default Panic");

    loop {}
}

/// On-panic function
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}




// main.rs

#![no_std] // don't use Rust standard library
#![no_main] // disable Rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

/*
* Replace system entry point, cannot use main() because part of C runtime
* Disable name mangling to ensure Rust compiler outputs function with name _start
* extern "C" tells compiler to use C calling convention because _start is default entry point for most systems
*/
#[unsafe(no_mangle)]
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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
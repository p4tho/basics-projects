// main.rs
#![no_std] // don't use Rust standard library
#![no_main] // disable Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

/*
* 1st
* Replace system entry point, cannot use main() because part of C runtime
* Disable name mangling to ensure Rust compiler outputs function with name _start
* extern "C" tells compiler to use C calling convention because _start is default entry point for most systems
*/
// static HELLO: &[u8] = b"Hello World!";
// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {
//     let vga_buffer = 0xb8000 as *mut u8;

//     for (i, &byte) in HELLO.iter().enumerate() {
//         unsafe {
//             *vga_buffer.offset(i as isize * 2) = byte;
//             *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
//         }
//     }

//     loop {}
// }

/* 
* 2nd 
*/
// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {
//     use core::fmt::Write;
//     vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
//     write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

//     loop {}
// }

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
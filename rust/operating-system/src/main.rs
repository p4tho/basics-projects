// main.rs
#![no_std] // don't use Rust standard library
#![no_main] // disable Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(operating_system::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use operating_system::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    operating_system::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }
    // trigger a stack overflow
    stack_overflow();

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    operating_system::test_panic_handler(info)
}

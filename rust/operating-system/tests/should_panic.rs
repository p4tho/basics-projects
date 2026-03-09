// should_panic.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use operating_system::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

/*
 * Test cases where it should result in a panic
 * Rust has #[should_panic], but it requires std library
 * Instead the panic handler will just have a success exit code
 * This is possible because each integration test file in tests/ is compiled separately
*/
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

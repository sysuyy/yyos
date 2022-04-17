#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
//#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(yy_os::test_runner)]

use core::panic::PanicInfo;
use yy_os::{println, serial_print, serial_println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    yy_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}

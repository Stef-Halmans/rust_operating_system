#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(crate::test::run::test_runner)]
#![reexport_test_harness_main = "test_main"]


pub mod serial;
pub mod test;
pub mod vga;
pub mod interrupts;
pub mod gdt;

#[cfg(test)]
use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use test::run::test_panic_handler;

    test_panic_handler(info)
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    test_main();

    loop {}
}


pub fn init() {
    interrupts::init();
    gdt::init();
}

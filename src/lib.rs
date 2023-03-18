#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(crate::test::run::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod test;
pub mod vga;

#[cfg(test)]
use core::panic::PanicInfo;

use x86_64::instructions;

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
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    println!("start hlt loop");
    loop{
        instructions::hlt();
    }
}

pub fn init() {
    gdt::init();
    interrupts::init();
    unsafe { interrupts::PICS.lock().initialize() };
    instructions::interrupts::enable();
}

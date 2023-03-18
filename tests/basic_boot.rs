
#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(operating_system::test::run::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use operating_system::println;
use operating_system::test::run::{test_panic_handler, self, Testable};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}


fn test_runner(tests: &[&dyn Testable]) {
    run::test_runner(tests)
}


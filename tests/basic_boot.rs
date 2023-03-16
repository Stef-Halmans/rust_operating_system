
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

use operating_system::vga::writer::{WRITER, BUFFER_HEIGHT};

const TEST_MANY_RUNS: usize = 200;

#[test_case]
fn test_println() {
    println!("test_println output")
}

#[test_case]
fn test_println_many() {
    for _ in 0..TEST_MANY_RUNS {
        println!("test_println output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Test string";
    println!("{}", s);

    for (col, c) in s.chars().enumerate() {
        // BUFFER_HEIGHT - 2 because index is - 1, and dont write to last row. 
        // col + one, because WRITER start writing at column 1 not 0
    
        assert_eq!(WRITER.lock().read_char(BUFFER_HEIGHT - 2, col + 1), c)
    }
}

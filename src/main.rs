#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod test;
mod exit;
mod serial;

use core::panic::PanicInfo;

use crate::exit::exit_qemu;




#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::exit::QemuExitCode;

    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world {}", 100);

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn assertion(){
    assert_eq!(2,2);
}

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use operating_system::{
    serial_print,
    test::exit::{
        exit_qemu,
        QemuExitCode::Success,
    }, serial_println,
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("panic message: {}", info);
    serial_print!("[ok]");
    exit_qemu(Success);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    should_fail();

    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...");
    assert_eq!(1, 0);

}



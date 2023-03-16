use core::{any::type_name, panic::PanicInfo};

use crate::{serial_print, serial_println, test::exit::{QemuExitCode, exit_qemu}, println};


pub fn test_runner(tests: &[&dyn Testable]) {
    println!("test");

    serial_println!("Running {} tests", tests.len());

    for test in tests{
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}


pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub trait Testable{
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn(), {

    fn run(&self) -> () {
        serial_print!("{}...\t", type_name::<T>());
        self();
        serial_println!("[ok]");
    }

}



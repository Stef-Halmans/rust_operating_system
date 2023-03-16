use core::any::type_name;

use crate::{serial_print, serial_println};


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::{exit::{QemuExitCode, exit_qemu}, serial_println};

    serial_println!("Running {} tests", tests.len());

    for test in tests{
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
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


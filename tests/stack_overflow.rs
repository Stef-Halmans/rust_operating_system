#![no_std]
#![no_main]

use core::panic::PanicInfo;

use operating_system::serial_print;
use volatile::Volatile;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");
    
    operating_system::init();
    operating_system::interrupts::init_test();

    stack_overflow();

    loop{}


}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    operating_system::test::run::test_panic_handler(info);
}

#[allow(unconditional_recursion)]
fn stack_overflow(){
    stack_overflow();
    Volatile::new(0).read();
}

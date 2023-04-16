#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::run::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod test;
mod serial;

use core::panic::PanicInfo;

use operating_system::hlt_loop;


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use operating_system::test::run::test_panic_handler;

    test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world {}!", 100);

    operating_system::init();
    println!("init finished");

    let ptr = 0x1240923490 as *mut u32;
    unsafe {*ptr = 42;}

    #[cfg(test)]
    test_main();

    hlt_loop();
}

#[test_case]
fn assertion(){
    assert_eq!(2,2);

}

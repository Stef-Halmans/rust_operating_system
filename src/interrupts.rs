use lazy_static::lazy_static;
use volatile::Volatile;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{gdt::DOUBLE_FAULT_IST_INDEX, println};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);

        let double_fault_entry_options = idt.double_fault.set_handler_fn(double_fault_handler);

        unsafe {
            double_fault_entry_options.set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };

}

lazy_static!{
    static ref IDT_TEST: InterruptDescriptorTable = {
        let mut idt:InterruptDescriptorTable = InterruptDescriptorTable::new();
        let double_fault_entry_options = idt.double_fault.set_handler_fn(double_fault_handler_test);

        unsafe {
            double_fault_entry_options.set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };

}



pub fn init() {
    IDT.load();
}

pub fn init_test(){
    IDT_TEST.load();

}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler_test(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    use crate::{serial_println, test::exit::{exit_qemu, QemuExitCode}};

    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop{}
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

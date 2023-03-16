use core::fmt::{Arguments, Write};

use crate::vga::writer::WRITER;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print("\n")); 
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments){
    WRITER.lock().write_fmt(args).expect("Issue running print macro");
}


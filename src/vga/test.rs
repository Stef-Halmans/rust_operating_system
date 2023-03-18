
use x86_64::instructions::interrupts;

use crate::{println, vga::writer::{WRITER, BUFFER_HEIGHT}};

const TEST_MANY_RUNS: usize = 200;

#[test_case]
fn test_println() {
    println!("test_println output");
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

    interrupts::without_interrupts(||{
        let mut writer = WRITER.lock();

        for (col, c) in s.chars().enumerate() {
            // BUFFER_HEIGHT - 2 because index is - 1, and dont write to last row. 
            // col + one, because WRITER start writing at column 1 not 0
        
            assert_eq!(writer.read_char(BUFFER_HEIGHT - 2, col + 1), c)
        }
        
    });

}

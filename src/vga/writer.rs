use core::fmt;

use spin::Mutex;

use lazy_static::lazy_static;
use volatile::Volatile;

pub const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const BUFFER_ADDRESS: i32 = 0xb8000;
const UNKNOWN_CHAR: u8 = 0xfe;
const EMPTY_CHAR: u8 = b' ';

const EMPTY_SCREEN_CHAR: ScreenChar = ScreenChar {
            char: EMPTY_CHAR,
            color_code: ColorCode::new(Color::Black, Color::Black),
        };

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(
        Writer::new(1, Color::White, Color::Black).expect("buffer start posision out of bounds")
    );
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    start_x: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(
        start_x: usize,
        foreground_color: Color,
        background_color: Color,
    ) -> Result<Writer, ()> {
        if start_x >= BUFFER_WIDTH {
            return Err(());
        }
        Ok(Writer {
            column_position: start_x,
            row_position: BUFFER_HEIGHT - 1,
            start_x,
            color_code: ColorCode::new(foreground_color, background_color),
            buffer: Buffer::new(),
        })
    }

    pub fn write_string(&mut self, string: &str) {
        for char in string.bytes() {
            match char {
                // valid ascii byte or newline
                0x20..=0x7e | b'\n' => self.write_char(char),
                _ => self.write_char(UNKNOWN_CHAR),
            }
        }
    }

    pub fn write_char(&mut self, char: u8) {
        match char {
            b'\n' => self.new_line(),
            char => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                self.buffer.chars[self.row_position][self.column_position].write(ScreenChar {
                    char,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }


    // only used for testing, otherwise no reading from vga_buffer allowed/needed
    pub fn read_char(&mut self, row: usize, col: usize) -> char{
        char::from(self.buffer.chars[row][col].read().char)
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char);
            }
        }

        self.clear_line(BUFFER_HEIGHT - 1);
        self.column_position = self.start_x;
    }

    fn clear_line(&mut self, row: usize) {
        for col in self.start_x..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(EMPTY_SCREEN_CHAR);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    fn new() -> &'static mut Buffer {
        unsafe { &mut *(BUFFER_ADDRESS as *mut Buffer) }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScreenChar {
    char: u8,
    color_code: ColorCode,
}

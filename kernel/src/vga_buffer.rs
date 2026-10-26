use core::clone::Clone;
use core::cmp::{Eq, PartialEq};
use core::derive;
use core::fmt::Debug;
use core::marker::Copy;
use core::ptr::{read_volatile, write_volatile};
use core::result::Result::Ok;

const BUFFER_COLUMN: usize = 80;
const BUFFER_ROW: usize = 25;
pub static mut CURSOR_ROW: usize = 0;
pub static mut CURSOR_COL: usize = 0;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Color {
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
    LightMagenta = 13,
    LightYellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorBF(u8);

impl ColorBF {
    fn new(fore: Color, back: Color) -> Self {
        ColorBF((back as u8) << 4 | (fore as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    attribute: ColorBF,
}

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_COLUMN]; BUFFER_ROW],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorBF,
    buffer: &'static mut Buffer,
}

impl Writer {
    #[allow(unused)]
    pub fn new() -> Self {
        Writer {
            column_position: 0,
            color_code: ColorBF::new(Color::Green, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }
    pub fn new_offset(offset: usize) -> Self {
        Writer {
            column_position: 0,
            color_code: ColorBF::new(Color::Green, Color::Black),
            buffer: unsafe { &mut *((0xb8000 + offset) as *mut Buffer) },
        }
    }

    pub fn byte_writer(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),

            byte => {
                //check position
                if self.column_position >= BUFFER_COLUMN {
                    self.new_line();
                }
                let row = BUFFER_ROW - 1;
                let column = self.column_position;
                let color_code = self.color_code;
                let screen_char = ScreenChar {
                    attribute: color_code,
                    character: byte,
                };
                unsafe {
                    let ptr = &mut self.buffer.chars[row][column] as *mut ScreenChar;
                    write_volatile(ptr, screen_char);
                }
                self.column_position += 1;
            }
        }
    }
    pub fn byte_writer_cursor_mvmnt(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position == BUFFER_COLUMN {
                    self.new_line();
                }
                let row = BUFFER_ROW - 1;
                let column = self.column_position;
                let color_code = self.color_code;
                let screen_char = ScreenChar {
                    attribute: color_code,
                    character: byte,
                };
                unsafe {
                    let ptr = &mut self.buffer.chars[row][column] as *mut ScreenChar;
                    write_volatile(ptr, screen_char);
                    CURSOR_COL += 1;
                    if CURSOR_COL >= 80 {
                        CURSOR_COL = 0;
                        CURSOR_ROW += 1;
                    }
                }
                self.column_position += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.byte_writer(byte),
                _ => self.byte_writer(0xfe),
            }
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_ROW {
            for col in 0..BUFFER_COLUMN {
                unsafe {
                    let source_ptr = &self.buffer.chars[row][col] as *const ScreenChar;
                    let character = read_volatile(source_ptr);

                    let write = &mut self.buffer.chars[row - 1][col] as *mut ScreenChar;
                    write_volatile(write, character);
                }
            }
            self.clear_row(BUFFER_ROW - 1);
            self.column_position = 0;
        }
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            character: b' ',
            attribute: self.color_code,
        };
        for col in 0..BUFFER_COLUMN {
            unsafe {
                let write = &mut self.buffer.chars[row][col] as *mut ScreenChar;
                write_volatile(write, blank);
            }
        }
    }
}

use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

//to fix print! macro doesn't work without a \n character for now use static mut for Writer::new to get desired outcome
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
//todo: prevent chances of a deadlock where writer is locked to one thing such as timer and cannot be used in case of interrupt
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    Writer::new().write_fmt(args).unwrap();
}

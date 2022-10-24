use crate::vga_buffer::vga_buffer::*;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

pub struct Writer {
    colum_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let characters = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(characters);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.colum_position = 0;
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.colum_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row: usize = BUFFER_HEIGHT - 1;
                let col: usize = self.colum_position;

                let color_code: ColorCode = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.colum_position += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                //printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static!{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        colum_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}


#[macro_export]
macro_rules! print {
        ($($arg:tt)*) => ($crate::vga_buffer::writer::_print(format_args!($($arg)*)));
    }

#[macro_export]
macro_rules! println {
        () => ($crate::print!("\n"));
        ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
    }

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

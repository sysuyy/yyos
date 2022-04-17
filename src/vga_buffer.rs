#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color 
{
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

impl ColorCode
{
    fn new(foreground: Color, background: Color) -> ColorCode
    {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar 
{
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;
#[repr(transparent)]
struct Buffer
{
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


pub struct Writer
{
    pointer: &'static mut Buffer,
    color_code: ColorCode,
    col_pos: usize,
}

impl Writer
{
    pub fn write_byte(&mut self, byte: u8)
    {
        match byte
        {
            b'\n' => self.new_line(),
            byte => 
            {
                if self.col_pos >= BUFFER_WIDTH
                {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.col_pos;

                let color_code = self.color_code;
                self.pointer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.col_pos += 1;
            }
        }
    }


    pub fn write_string(&mut self, s: &str)
    {
        for byte in s.bytes()
        {
            match byte
            {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self)
    {
       for i in 0..BUFFER_HEIGHT-1 
       {
            for j in 0..BUFFER_WIDTH
            {
                self.pointer.chars[i][j].write(self.pointer.chars[i+1][j].read());
            }
       }
       self.col_pos = 0;
       self.clean_row();
       self.col_pos = 0;
    }

    fn clean_row(&mut self)
    {
        for _col in 0..BUFFER_WIDTH
        {
            self.write_byte(b' ');
        }
    }
}

use core::fmt;

impl fmt::Write for Writer
{
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
        self.write_string(s);
        Ok(())
    }
}
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer{
        col_pos: 0,
        color_code: ColorCode::new(Color::LightCyan, Color::Black),
        pointer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
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

#[cfg(test)]
use crate::{serial_print, serial_println};

#[test_case]
fn test_println_simple() {
    serial_print!("test_println... ");
    println!("test_println_simple output");
    serial_println!("[ok]");
}

#[test_case]
fn test_println_output() {
    serial_print!("test_println_output... ");

    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().pointer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }

    serial_println!("[ok]");
}

use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
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
    White = 15
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground:Color,background:Color)->ColorCode{
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(C)]
pub struct ScreenChar{
    pub ascii_character: u8,
    pub color_code:ColorCode
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer{
    pub chars:[[Volatile<ScreenChar>;BUFFER_WIDTH];BUFFER_HEIGHT]
}

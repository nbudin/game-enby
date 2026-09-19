use bitfield_struct::bitenum;

use crate::ppu::registers::{LCDControlRegister, LCDStatusRegister};

pub mod registers;

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
#[bitenum]
pub enum PPUMode {
    #[fallback]
    HorizontalBlank = 0,
    VerticalBlank = 1,
    OAMScan = 2,
    DrawingPixels = 3,
}

pub struct PPU {
    pub lcd_control: LCDControlRegister,
    pub lcd_status: LCDStatusRegister,
    pub bg_viewport_x: u8,
    pub bg_viewport_y: u8,
    pub ly: u8,
    pub lyc: u8,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            lcd_control: LCDControlRegister::new(),
            lcd_status: LCDStatusRegister::new(),
            bg_viewport_x: 0,
            bg_viewport_y: 0,
            ly: 0,
            lyc: 0,
        }
    }
}

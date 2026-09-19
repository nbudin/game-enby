use bitfield_struct::bitenum;

use crate::ppu::registers::LCDStatusRegister;

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
    pub lcd_status: LCDStatusRegister,
}

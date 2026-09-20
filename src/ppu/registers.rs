use bitfield_struct::{bitenum, bitfield};

use crate::ppu::PPUMode;

#[bitfield(u8)]
pub struct LCDControlRegister {
    pub bg_window_enable_priority: bool,
    pub obj_enable: bool,
    pub obj_size: bool,
    pub bg_tile_map: bool,
    pub bg_window_tiles: bool,
    pub window_enable: bool,
    pub window_tile_map: bool,
    pub lcd_ppu_enable: bool,
}

#[bitfield(u8)]
pub struct LCDStatusRegister {
    #[bits(2)]
    pub ppu_mode: PPUMode,
    pub lyc_int_select: bool,
    pub mode0_int_select: bool,
    pub mode1_int_select: bool,
    pub mode2_int_select: bool,
    pub lyc_equals_ly: bool,
    _unused: bool,
}

impl LCDStatusRegister {
    pub fn write_from_bus(&self, value: LCDStatusRegister) -> LCDStatusRegister {
        self.with_lyc_int_select(value.lyc_int_select())
            .with_mode0_int_select(value.mode0_int_select())
            .with_mode1_int_select(value.mode1_int_select())
            .with_mode2_int_select(value.mode2_int_select())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[bitenum]
pub enum GreyscaleColor {
    #[fallback]
    White = 0,
    LightGrey = 1,
    DarkGrey = 2,
    Black = 3,
}

#[bitfield(u8)]
pub struct GreyscalePaletteData {
    #[bits(2)]
    pub color0: GreyscaleColor,
    #[bits(2)]
    pub color1: GreyscaleColor,
    #[bits(2)]
    pub color2: GreyscaleColor,
    #[bits(2)]
    pub color3: GreyscaleColor,
}

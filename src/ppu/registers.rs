use bitfield_struct::bitfield;

use crate::ppu::PPUMode;

#[bitfield(u8)]
pub struct LCDControlRegister {
    bg_window_enable_priority: bool,
    obj_enable: bool,
    obj_size: bool,
    bg_tile_map: bool,
    bg_window_tiles: bool,
    window_enable: bool,
    window_tile_map: bool,
    lcd_ppu_enable: bool,
}

#[bitfield(u8)]
pub struct LCDStatusRegister {
    #[bits(2)]
    ppu_mode: PPUMode,
    lyc_int_select: bool,
    mode0_int_select: bool,
    mode1_int_select: bool,
    mode2_int_select: bool,
    lyc_equals_ly: bool,
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

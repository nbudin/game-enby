use bitfield_struct::bitenum;

use crate::ppu::{
    oam::OAMEntry,
    registers::{GreyscalePaletteData, LCDControlRegister, LCDStatusRegister},
    tile::TileData,
};

pub mod oam;
pub mod registers;
pub mod tile;

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
    pub bg_palette_data: GreyscalePaletteData,
    pub obj_palette0_data: GreyscalePaletteData,
    pub obj_palette1_data: GreyscalePaletteData,
    pub tile_data_block0: [TileData; 128],
    pub tile_data_block1: [TileData; 128],
    pub tile_data_block2: [TileData; 128],
    pub tilemap0: [u8; 1024],
    pub tilemap1: [u8; 1024],
    pub oam: [OAMEntry; 40],
    pub scx: u8,
    pub scy: u8,
    pub ly: u8,
    pub lyc: u8,

    mode_wait_cycles: usize,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            lcd_control: LCDControlRegister::new(),
            lcd_status: LCDStatusRegister::new().with_ppu_mode(PPUMode::OAMScan),
            bg_palette_data: GreyscalePaletteData::new(),
            obj_palette0_data: GreyscalePaletteData::new(),
            obj_palette1_data: GreyscalePaletteData::new(),
            tile_data_block0: [TileData([0; _]); _],
            tile_data_block1: [TileData([0; _]); _],
            tile_data_block2: [TileData([0; _]); _],
            tilemap0: [0; _],
            tilemap1: [0; _],
            oam: [OAMEntry::new(); _],
            scx: 0,
            scy: 0,
            ly: 0,
            lyc: 0,
            mode_wait_cycles: 80,
        }
    }

    pub fn tick(&mut self) {
        match self.lcd_status.ppu_mode() {
            PPUMode::HorizontalBlank => {
                if self.mode_wait_cycles == 0 {
                    self.ly += 1;

                    if self.ly < 144 {
                        self.lcd_status.set_ppu_mode(PPUMode::OAMScan);
                        self.mode_wait_cycles = 80;
                    } else {
                        self.lcd_status.set_ppu_mode(PPUMode::VerticalBlank);
                        self.mode_wait_cycles = 4560;
                    }
                } else {
                    self.mode_wait_cycles -= 1;
                }
            }
            PPUMode::VerticalBlank => {
                if self.mode_wait_cycles == 0 {
                    self.ly = 0;
                    self.lcd_status.set_ppu_mode(PPUMode::OAMScan);
                    self.mode_wait_cycles = 80;
                } else {
                    if self.mode_wait_cycles % 456 == 455 {
                        self.ly += 1;
                    }
                    self.mode_wait_cycles -= 1;
                }
            }
            PPUMode::OAMScan => {
                if self.mode_wait_cycles == 0 {
                    self.lcd_status.set_ppu_mode(PPUMode::DrawingPixels);
                    self.mode_wait_cycles = 172; // TODO: Penalties
                } else {
                    self.mode_wait_cycles -= 1;
                }
            }
            PPUMode::DrawingPixels => {
                if self.mode_wait_cycles == 0 {
                    self.lcd_status.set_ppu_mode(PPUMode::HorizontalBlank);
                    self.mode_wait_cycles = 204; // TODO: Penalties
                } else {
                    self.mode_wait_cycles -= 1;
                }
            }
        }

        self.lcd_status.set_lyc_equals_ly(self.lyc == self.ly);
    }
}

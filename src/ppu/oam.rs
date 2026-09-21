use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct OAMAttributes {
    #[bits(3)]
    cgb_palette: u8,
    #[bits(1)]
    cgb_bank: u8,
    #[bits(1)]
    dmb_palette: u8,
    x_flip: bool,
    y_flip: bool,
    priority: bool,
}

#[bitfield(u32)]
pub struct OAMEntry {
    #[bits(8)]
    pub attributes: OAMAttributes,
    pub tile_index: u8,
    pub x_position_minus_8: u8,
    pub y_position_minus_16: u8,
}

impl OAMEntry {
    pub fn read_byte(&self, byte_index: usize) -> u8 {
        match byte_index {
            0 => self.attributes().into_bits(),
            1 => self.tile_index(),
            2 => self.x_position_minus_8(),
            3 => self.y_position_minus_16(),
            _ => panic!("Invalid byte index {}", byte_index),
        }
    }

    pub fn write_byte(&mut self, byte_index: usize, value: u8) {
        match byte_index {
            0 => self.set_attributes(OAMAttributes::from_bits(value)),
            1 => self.set_tile_index(value),
            2 => self.set_x_position_minus_8(value),
            3 => self.set_y_position_minus_16(value),
            _ => panic!("Invalid byte index {}", byte_index),
        }
    }
}

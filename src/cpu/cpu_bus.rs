use crate::bus::Bus;

pub struct CPUBus {
    work_ram: Vec<u8>,
}

impl CPUBus {
    pub fn new() -> CPUBus {
        CPUBus {
            // TODO: support CGB bank switching
            work_ram: Vec::with_capacity(2048),
        }
    }
}

impl Bus<u16> for CPUBus {
    fn try_read_readonly(&self, addr: u16) -> Option<u8> {
        match addr {
            0x0000..=0x7FFF => todo!("Cartridge ROM"),
            0x8000..=0x9FFF => todo!("VRAM"),
            0xA000..=0xBFFF => todo!("External RAM"),
            0xC000..=0xDFFF => Some(self.work_ram[(addr as usize) - 0xC000]),
            0xE000..=0xFDFF => self.try_read_readonly(addr - 0x2000),
            0xFE00..=0xFE9F => todo!("OAM"),
            0xFEA0..=0xFEFF => todo!("Not usable"),
            0xFF00..=0xFF7F => todo!("I/O registers"),
            0xFF80..=0xFFFE => todo!("High RAM"),
            0xFFFF => todo!("IE register"),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        todo!()
    }
}

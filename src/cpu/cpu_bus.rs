use std::sync::{Arc, RwLock};

use crate::{
    apu::{
        APU,
        registers::{
            AudioMasterControlRegister, AudioMasterVolumeVINPanningRegister, AudioPanningRegister,
        },
    },
    bus::{Bus, bus_interceptor::BusInterceptor},
    cpu::{
        CPU,
        registers::{IERegister, IFRegister},
    },
    ppu::{
        PPU,
        registers::{GreyscalePaletteData, LCDControlRegister, LCDStatusRegister},
    },
};

pub trait CPUBusTrait: Bus<u16> {}

pub struct CPUBus {
    work_ram: Vec<u8>,
    high_ram: [u8; 127],
    pub apu: Arc<RwLock<APU>>,
    pub cpu: Arc<RwLock<CPU>>,
    pub ppu: Arc<RwLock<PPU>>,
}

impl CPUBus {
    pub fn new(apu: Arc<RwLock<APU>>, cpu: Arc<RwLock<CPU>>, ppu: Arc<RwLock<PPU>>) -> CPUBus {
        CPUBus {
            // TODO: support CGB bank switching
            work_ram: [0; 8192].to_vec(),
            high_ram: [0; _],
            apu,
            cpu,
            ppu,
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
            0xFF0F => Some(
                self.cpu
                    .read()
                    .unwrap()
                    .registers
                    .interrupt_flag
                    .into_bits(),
            ),
            0xFF24 => Some(
                self.apu
                    .read()
                    .unwrap()
                    .master_volume_vin_panning
                    .into_bits(),
            ),
            0xFF25 => Some(self.apu.read().unwrap().panning.into_bits()),
            0xFF26 => Some(self.apu.read().unwrap().master_control.into_bits()),
            0xFF40 => Some(self.ppu.read().unwrap().lcd_control.into_bits()),
            0xFF41 => Some(self.ppu.read().unwrap().lcd_status.into_bits()),
            0xFF42 => Some(self.ppu.read().unwrap().scy),
            0xFF43 => Some(self.ppu.read().unwrap().scx),
            0xFF44 => Some(self.ppu.read().unwrap().ly),
            0xFF47 => Some(self.ppu.read().unwrap().bg_palette_data.into_bits()),
            0xFF48 => Some(self.ppu.read().unwrap().obj_palette0_data.into_bits()),
            0xFF49 => Some(self.ppu.read().unwrap().obj_palette1_data.into_bits()),
            0xFF00..=0xFF7F => todo!("I/O register {:04X}", addr),
            0xFF80..=0xFFFE => Some(self.high_ram[(addr - 0xFF80) as usize]),
            0xFFFF => Some(
                self.cpu
                    .read()
                    .unwrap()
                    .registers
                    .interrupt_enable
                    .into_bits(),
            ),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => todo!("Cartridge ROM"),
            0x8000..=0x9FFF => todo!("VRAM"),
            0xA000..=0xBFFF => todo!("External RAM"),
            0xC000..=0xDFFF => self.work_ram[(addr as usize) - 0xC000] = value,
            0xE000..=0xFDFF => self.write(addr - 0x2000, value),
            0xFE00..=0xFE9F => todo!("OAM"),
            0xFEA0..=0xFEFF => todo!("Not usable"),
            0xFF0F => {
                self.cpu.write().unwrap().registers.interrupt_flag = IFRegister::from_bits(value)
            }
            0xFF24 => {
                self.apu.write().unwrap().master_volume_vin_panning =
                    AudioMasterVolumeVINPanningRegister::from_bits(value)
            }
            0xFF25 => self.apu.write().unwrap().panning = AudioPanningRegister::from_bits(value),
            0xFF26 => {
                self.apu.write().unwrap().master_control =
                    AudioMasterControlRegister::from_bits(value)
            }
            0xFF40 => self.ppu.write().unwrap().lcd_control = LCDControlRegister::from_bits(value),
            0xFF41 => {
                let new_value = self
                    .ppu
                    .read()
                    .unwrap()
                    .lcd_status
                    .write_from_bus(LCDStatusRegister::from_bits(value));
                self.ppu.write().unwrap().lcd_status = new_value;
            }
            0xFF42 => {
                // TODO: Delayed writes https://gbdev.io/pandocs/Scrolling.html#viewport-position-scrolling
                self.ppu.write().unwrap().scy = value;
            }
            0xFF43 => {
                // TODO: Delayed writes https://gbdev.io/pandocs/Scrolling.html#viewport-position-scrolling
                self.ppu.write().unwrap().scx = value;
            }
            0xFF44 => {}
            0xFF47 => {
                self.ppu.write().unwrap().bg_palette_data = GreyscalePaletteData::from_bits(value)
            }
            0xFF48 => {
                self.ppu.write().unwrap().obj_palette0_data = GreyscalePaletteData::from_bits(value)
            }
            0xFF49 => {
                self.ppu.write().unwrap().obj_palette1_data = GreyscalePaletteData::from_bits(value)
            }
            0xFF00..=0xFF7F => todo!("I/O register {:04X}", addr),
            0xFF80..=0xFFFE => self.high_ram[(addr - 0xFF80) as usize] = value,
            0xFFFF => {
                self.cpu.write().unwrap().registers.interrupt_enable = IERegister::from_bits(value)
            }
        }
    }
}

impl CPUBusTrait for CPUBus {}

impl<T> CPUBusTrait for T where T: BusInterceptor<u16, BusType = CPUBus> {}

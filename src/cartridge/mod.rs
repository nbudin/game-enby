use std::sync::{Arc, RwLock};

use enum_dispatch::enum_dispatch;

use crate::{
    cartridge::no_mbc::NoMBC,
    cpu::{CPU, cpu_bus::CPUBusTrait},
    ppu::PPU,
};

pub mod no_mbc;

#[enum_dispatch]
pub trait CartridgeBehavior {
    fn cpu_bus(&self) -> &dyn CPUBusTrait;
    fn cpu_bus_mut(&mut self) -> &mut dyn CPUBusTrait;
}

#[enum_dispatch(CartridgeBehavior)]
pub enum Cartridge {
    NoMBC,
}

impl Cartridge {
    pub fn from_rom(rom: Vec<u8>, cpu: Arc<RwLock<CPU>>, ppu: Arc<RwLock<PPU>>) -> Self {
        NoMBC::from_rom(rom, cpu, ppu).into()
    }
}

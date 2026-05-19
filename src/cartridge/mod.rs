use enum_dispatch::enum_dispatch;

use crate::{bus::Bus, cartridge::no_mbc::NoMBC};

pub mod no_mbc;

#[enum_dispatch]
pub trait CartridgeBehavior {
    fn cpu_bus(&self) -> &dyn Bus<u16>;
    fn cpu_bus_mut(&mut self) -> &mut dyn Bus<u16>;
}

#[enum_dispatch(CartridgeBehavior)]
pub enum Cartridge {
    NoMBC,
}

impl Cartridge {
    pub fn from_rom(rom: Vec<u8>) -> Self {
        NoMBC::from_rom(rom).into()
    }
}

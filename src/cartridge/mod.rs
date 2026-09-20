use std::sync::{Arc, RwLock};

use enum_dispatch::enum_dispatch;

use crate::{
    apu::APU,
    cartridge::{header::CartridgeHeader, no_mbc::NoMBC},
    cpu::{CPU, cpu_bus::CPUBusTrait},
    ppu::PPU,
};

pub mod header;
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
    pub fn from_rom(
        rom: Vec<u8>,
        apu: Arc<RwLock<APU>>,
        cpu: Arc<RwLock<CPU>>,
        ppu: Arc<RwLock<PPU>>,
    ) -> Self {
        let header = CartridgeHeader::from_rom(&rom).unwrap();

        println!("{:?}", header);

        match header.cartridge_type {
            header::CartridgeType::NoMBC => NoMBC::from_rom(rom, apu, cpu, ppu).into(),
            _ => todo!("{}", header.cartridge_type.as_ref()),
        }
    }
}

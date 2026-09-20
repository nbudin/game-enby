use std::sync::{Arc, RwLock};

use crate::{
    apu::APU,
    bus::bus_interceptor::{BusInterceptor, InterceptorResult},
    cartridge::CartridgeBehavior,
    cpu::{
        CPU,
        cpu_bus::{CPUBus, CPUBusTrait},
    },
    ppu::PPU,
};

pub struct NoMBCCPUBusInterceptor {
    rom: Vec<u8>,
    bus: CPUBus,
}

impl BusInterceptor<u16> for NoMBCCPUBusInterceptor {
    type BusType = CPUBus;

    fn get_inner(&self) -> &Self::BusType {
        &self.bus
    }

    fn get_inner_mut(&mut self) -> &mut Self::BusType {
        &mut self.bus
    }

    fn intercept_read_readonly(&self, addr: u16) -> InterceptorResult<Option<u8>> {
        if addr <= 0x7FFF {
            InterceptorResult::Intercepted(self.rom.get(addr as usize).copied())
        } else {
            InterceptorResult::NotIntercepted
        }
    }

    fn intercept_write(&mut self, _addr: u16, _value: u8) -> InterceptorResult<()> {
        InterceptorResult::NotIntercepted
    }
}

pub struct NoMBC {
    cpu_bus: NoMBCCPUBusInterceptor,
}

impl NoMBC {
    pub fn from_rom(
        rom: Vec<u8>,
        apu: Arc<RwLock<APU>>,
        cpu: Arc<RwLock<CPU>>,
        ppu: Arc<RwLock<PPU>>,
    ) -> Self {
        NoMBC {
            cpu_bus: NoMBCCPUBusInterceptor {
                rom,
                bus: CPUBus::new(apu, cpu, ppu),
            },
        }
    }
}

impl CartridgeBehavior for NoMBC {
    fn cpu_bus(&self) -> &dyn CPUBusTrait {
        &self.cpu_bus
    }

    fn cpu_bus_mut(&mut self) -> &mut dyn CPUBusTrait {
        &mut self.cpu_bus
    }
}

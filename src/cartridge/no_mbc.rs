use crate::{
    bus::{
        Bus,
        bus_interceptor::{BusInterceptor, InterceptorResult},
    },
    cartridge::CartridgeBehavior,
    cpu::cpu_bus::CPUBus,
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

    fn intercept_write(&mut self, addr: u16, value: u8) -> InterceptorResult<()> {
        InterceptorResult::NotIntercepted
    }
}

pub struct NoMBC {
    cpu_bus: NoMBCCPUBusInterceptor,
}

impl NoMBC {
    pub fn from_rom(rom: Vec<u8>) -> Self {
        NoMBC {
            cpu_bus: NoMBCCPUBusInterceptor {
                rom,
                bus: CPUBus::new(),
            },
        }
    }
}

impl CartridgeBehavior for NoMBC {
    fn cpu_bus(&self) -> &dyn Bus<u16> {
        &self.cpu_bus
    }

    fn cpu_bus_mut(&mut self) -> &mut dyn Bus<u16> {
        &mut self.cpu_bus
    }
}

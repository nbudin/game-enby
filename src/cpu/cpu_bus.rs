use crate::{
    bus::{Bus, bus_interceptor::BusInterceptor},
    cpu::registers::{
        AFRegister, BCRegister, CPURegisters, DERegister, HLRegister, IERegister, IFRegister,
    },
};

pub trait CPUBusTrait: Bus<u16> {
    fn registers(&self) -> &CPURegisters;
    fn registers_mut(&mut self) -> &mut CPURegisters;
    fn get_interrupt_master_enable(&self) -> bool;
    fn set_interrupt_master_enable(&mut self, value: bool);
}

pub struct CPUBus {
    work_ram: Vec<u8>,
    pub registers: CPURegisters,
    pub interrupt_master_enable: bool,
}

impl CPUBus {
    pub fn new() -> CPUBus {
        CPUBus {
            // TODO: support CGB bank switching
            work_ram: Vec::with_capacity(2048),
            registers: CPURegisters {
                af: AFRegister::from_bits(0),
                bc: BCRegister::from_bits(0),
                de: DERegister::from_bits(0),
                hl: HLRegister::from_bits(0),
                sp: 0,
                pc: 0x0100,
                interrupt_enable: IERegister::from_bits(0),
                interrupt_flag: IFRegister::from_bits(0),
            },
            interrupt_master_enable: false,
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
            0xFF00..=0xFF0E => todo!("I/O register {:04X}", addr),
            0xFF0F => Some(self.registers.interrupt_flag.into_bits()),
            0xFF10..=0xFF7F => todo!("I/O register {:04X}", addr),
            0xFF80..=0xFFFE => todo!("High RAM"),
            0xFFFF => Some(self.registers.interrupt_enable.into_bits()),
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
            0xFF00..=0xFF0E => todo!("I/O register {:04X}", addr),
            0xFF0F => self.registers.interrupt_flag = IFRegister::from_bits(value),
            0xFF10..=0xFF7F => todo!("I/O register {:04X}", addr),
            0xFF80..=0xFFFE => todo!("High RAM"),
            0xFFFF => self.registers.interrupt_enable = IERegister::from_bits(value),
        }
    }
}

impl CPUBusTrait for CPUBus {
    fn registers(&self) -> &CPURegisters {
        &self.registers
    }

    fn registers_mut(&mut self) -> &mut CPURegisters {
        &mut self.registers
    }

    fn get_interrupt_master_enable(&self) -> bool {
        self.interrupt_master_enable
    }

    fn set_interrupt_master_enable(&mut self, value: bool) {
        self.interrupt_master_enable = value;
    }
}

impl<T> CPUBusTrait for T
where
    T: BusInterceptor<u16, BusType = CPUBus>,
{
    fn registers(&self) -> &CPURegisters {
        self.get_inner().registers()
    }

    fn registers_mut(&mut self) -> &mut CPURegisters {
        self.get_inner_mut().registers_mut()
    }

    fn get_interrupt_master_enable(&self) -> bool {
        self.get_inner().get_interrupt_master_enable()
    }

    fn set_interrupt_master_enable(&mut self, value: bool) {
        self.get_inner_mut().set_interrupt_master_enable(value)
    }
}

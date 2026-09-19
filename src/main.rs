use std::{
    env,
    fmt::Display,
    fs::File,
    io::{Read, Write},
};

use crate::{
    cartridge::{Cartridge, CartridgeBehavior},
    cpu::{
        CPU,
        disasm::read_instruction,
        instructions::InstructionBehavior,
        registers::{BCRegister, CPUFlags, DERegister, HLRegister},
    },
};

mod bus;
mod cartridge;
mod cpu;
mod ppu;

pub struct TraceState {
    a: u8,
    f: CPUFlags,
    bc: BCRegister,
    de: DERegister,
    hl: HLRegister,
    sp: u16,
    pc: u16,
    pcmem: [u8; 4],
}

impl Display for TraceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "A: {:02X} F: {:02X}, BC: {:04X} DE: {:04X} HL: {:04X} SP: {:04X} PC: {:04X} PCMEM: {:02X},{:02X},{:02X},{:02X}",
                self.a,
                self.f.into_bits(),
                self.bc.into_bits(),
                self.de.into_bits(),
                self.hl.into_bits(),
                self.sp,
                self.pc,
                self.pcmem[0],
                self.pcmem[1],
                self.pcmem[2],
                self.pcmem[3]
            )
        )
    }
}

pub struct Machine {
    cpu: CPU,
    cartridge: Cartridge,
}

impl Machine {
    pub fn from_rom(rom: Vec<u8>) -> Machine {
        Machine {
            cpu: CPU::new(),
            cartridge: Cartridge::from_rom(rom),
        }
    }

    pub fn trace_state(&self) -> TraceState {
        let bus = self.cartridge.cpu_bus();
        let pcmem = [
            bus.read_readonly(bus.registers().pc),
            bus.read_readonly(bus.registers().pc + 1),
            bus.read_readonly(bus.registers().pc + 2),
            bus.read_readonly(bus.registers().pc + 3),
        ];
        TraceState {
            a: bus.registers().af.a(),
            f: bus.registers().af.f(),
            bc: bus.registers().bc,
            de: bus.registers().de,
            hl: bus.registers().hl,
            sp: bus.registers().sp,
            pc: bus.registers().pc,
            pcmem,
        }
    }
}

impl Read for Machine {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for i in 0..buf.len() {
            let pc = self.cartridge.cpu_bus().registers().pc;
            buf[i] = self.cartridge.cpu_bus_mut().read(pc);
            self.cartridge.cpu_bus_mut().registers_mut().pc += 1;
        }

        Ok(buf.len())
    }
}

fn main() {
    let args = env::args().into_iter().collect::<Vec<_>>();
    let mut rom = File::open(&args[1]).unwrap();
    let mut rom_data = vec![];
    rom.read_to_end(&mut rom_data).unwrap();

    let mut machine = Machine::from_rom(rom_data);

    for _i in 1..1000 {
        let instruction = read_instruction(&mut machine).unwrap();
        println!("{:20} {}", format!("{instruction}"), machine.trace_state());
        instruction.execute(&mut machine);
    }
}

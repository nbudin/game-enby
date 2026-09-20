use std::{
    env,
    fmt::Display,
    fs::File,
    io::Read,
    sync::{Arc, RwLock},
};

use crate::{
    apu::APU,
    cartridge::{Cartridge, CartridgeBehavior},
    cpu::{
        CPU,
        disasm::read_instruction,
        instructions::InstructionBehavior,
        registers::{BCRegister, CPUFlags, DERegister, HLRegister},
    },
    ppu::PPU,
};

mod apu;
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
    apu: Arc<RwLock<APU>>,
    cpu: Arc<RwLock<CPU>>,
    ppu: Arc<RwLock<PPU>>,
    cartridge: Cartridge,
    cpu_wait_cycles: usize,
}

impl Machine {
    pub fn from_rom(rom: Vec<u8>) -> Machine {
        let apu = Arc::new(RwLock::new(APU::new()));
        let cpu = Arc::new(RwLock::new(CPU::new()));
        let ppu = Arc::new(RwLock::new(PPU::new()));

        Machine {
            apu: apu.clone(),
            cpu: cpu.clone(),
            ppu: ppu.clone(),
            cartridge: Cartridge::from_rom(rom, apu, cpu, ppu),
            cpu_wait_cycles: 0,
        }
    }

    pub fn trace_state(&self) -> TraceState {
        let bus = self.cartridge.cpu_bus();
        let pcmem = [
            bus.read_readonly(self.cpu.read().unwrap().registers.pc),
            bus.read_readonly(self.cpu.read().unwrap().registers.pc + 1),
            bus.read_readonly(self.cpu.read().unwrap().registers.pc + 2),
            bus.read_readonly(self.cpu.read().unwrap().registers.pc + 3),
        ];
        TraceState {
            a: self.cpu.read().unwrap().registers.af.a(),
            f: self.cpu.read().unwrap().registers.af.f(),
            bc: self.cpu.read().unwrap().registers.bc,
            de: self.cpu.read().unwrap().registers.de,
            hl: self.cpu.read().unwrap().registers.hl,
            sp: self.cpu.read().unwrap().registers.sp,
            pc: self.cpu.read().unwrap().registers.pc,
            pcmem,
        }
    }

    pub fn tick(&mut self) {
        if self.cpu_wait_cycles == 0 {
            let instruction = read_instruction(self).unwrap();
            eprintln!("{:20} {}", format!("{instruction}"), self.trace_state());
            instruction.execute(self.cpu.clone(), self.cartridge.cpu_bus_mut());
            self.cpu_wait_cycles = instruction.duration() - 1;
        } else {
            self.cpu_wait_cycles -= 1;
        }

        self.ppu.write().unwrap().tick();
    }
}

impl Read for Machine {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for i in 0..buf.len() {
            let pc = self.cpu.read().unwrap().registers.pc;
            buf[i] = self.cartridge.cpu_bus_mut().read(pc);
            self.cpu.write().unwrap().registers.pc += 1;
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

    for _i in 1..100000 {
        machine.tick();
    }
}

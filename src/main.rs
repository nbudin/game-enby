use std::{
    env,
    fs::File,
    io::{Cursor, Read},
};

use crate::{
    cartridge::{Cartridge, CartridgeBehavior},
    cpu::{CPU, disasm::read_instruction, instructions::InstructionBehavior},
};

mod bus;
mod cartridge;
mod cpu;

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

    for _i in 1..10 {
        let instruction = read_instruction(&mut machine).unwrap();
        println!("{:?}", instruction);
        instruction.execute(&mut machine);
    }
}

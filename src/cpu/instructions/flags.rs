use std::{
    fmt::Display,
    io::Write,
    sync::{Arc, RwLock},
};

use crate::cpu::{CPU, asm::Assemble, cpu_bus::CPUBusTrait, instructions::InstructionBehavior};

#[derive(Debug)]
pub enum CCFInstruction {
    Empty,
}

impl InstructionBehavior for CCFInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, _cpu_bus: &mut dyn CPUBusTrait) {
        let registers = &mut cpu.write().unwrap().registers;
        registers.af.set_f(
            registers
                .af
                .f()
                .with_c(!registers.af.f().c())
                .with_n(false)
                .with_h(false),
        );
    }

    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for CCFInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x3F])
    }
}

impl Display for CCFInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CCF")
    }
}

#[derive(Debug)]
pub enum DIInstruction {
    Empty,
}

impl InstructionBehavior for DIInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, _cpu_bus: &mut dyn CPUBusTrait) {
        cpu.write().unwrap().interrupt_master_enable = false;
    }

    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for DIInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0xF3])
    }
}

impl Display for DIInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DI")
    }
}

#[derive(Debug)]
pub enum EIInstruction {
    Empty,
}

impl InstructionBehavior for EIInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, _cpu_bus: &mut dyn CPUBusTrait) {
        cpu.write().unwrap().interrupt_master_enable = true;
    }

    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for EIInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0xFB])
    }
}

impl Display for EIInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EI")
    }
}

#[derive(Debug)]
pub enum SCFInstruction {
    Empty,
}

impl InstructionBehavior for SCFInstruction {
    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for SCFInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x37])
    }
}

impl Display for SCFInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SCF")
    }
}

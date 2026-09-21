use std::{
    fmt::Display,
    io::Write,
    sync::{Arc, RwLock},
};

use crate::cpu::{
    CPU,
    cpu_bus::CPUBusTrait,
    instructions::{Assemble, BitOffset, InstructionBehavior},
    registers::Register8,
};

#[derive(Debug)]
pub enum BITInstruction {
    U3R8(BitOffset, Register8),
    U3HL(BitOffset),
}

impl InstructionBehavior for BITInstruction {
    fn duration(&self) -> usize {
        match self {
            BITInstruction::U3R8(_, _) => 8,
            BITInstruction::U3HL(_) => 12,
        }
    }
}

impl Assemble for BITInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            BITInstruction::U3R8(bit_offset, register8) => match register8 {
                Register8::B => dest.write(&[0xCB, 0x40 + (*bit_offset as u8) * 8]),
                Register8::C => dest.write(&[0xCB, 0x41 + (*bit_offset as u8) * 8]),
                Register8::D => dest.write(&[0xCB, 0x42 + (*bit_offset as u8) * 8]),
                Register8::E => dest.write(&[0xCB, 0x43 + (*bit_offset as u8) * 8]),
                Register8::H => dest.write(&[0xCB, 0x44 + (*bit_offset as u8) * 8]),
                Register8::L => dest.write(&[0xCB, 0x45 + (*bit_offset as u8) * 8]),
                Register8::A => dest.write(&[0xCB, 0x47 + (*bit_offset as u8) * 8]),
            },
            BITInstruction::U3HL(bit_offset) => dest.write(&[0xCB, 0x46 + (*bit_offset as u8) * 8]),
        }
    }
}

impl Display for BITInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BITInstruction::U3R8(bit_offset, register8) => f.write_fmt(format_args!(
                "BIT {}, {}",
                *bit_offset as u8,
                register8.as_ref()
            )),
            BITInstruction::U3HL(bit_offset) => {
                f.write_fmt(format_args!("BIT {}, [HL]", *bit_offset as u8))
            }
        }
    }
}

#[derive(Debug)]
pub enum CPInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl CPInstruction {
    fn compare_with_a(&self, cpu: Arc<RwLock<CPU>>, value: u8) {
        let mut cpu = cpu.write().unwrap();
        let a_value = cpu.registers.af.a();
        let new_f = cpu
            .registers
            .af
            .f()
            .with_z(a_value == value)
            .with_n(true)
            .with_h(value & 0x0F > a_value & 0x0F)
            .with_c(value > a_value);
        cpu.registers.af.set_f(new_f);
    }
}

impl InstructionBehavior for CPInstruction {
    fn duration(&self) -> usize {
        match self {
            CPInstruction::AR8(_) => 4,
            CPInstruction::AHL => 8,
            CPInstruction::AN8(_) => 8,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            CPInstruction::AR8(register8) => {
                let value = cpu.read().unwrap().registers.get_r8(*register8);
                self.compare_with_a(cpu, value);
            }
            CPInstruction::AHL => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                self.compare_with_a(cpu, value)
            }
            CPInstruction::AN8(value) => {
                self.compare_with_a(cpu, *value);
            }
        }
    }
}

impl Assemble for CPInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            CPInstruction::AR8(register8) => match register8 {
                Register8::B => dest.write(&[0xB8]),
                Register8::C => dest.write(&[0xB9]),
                Register8::D => dest.write(&[0xBA]),
                Register8::E => dest.write(&[0xBB]),
                Register8::H => dest.write(&[0xBC]),
                Register8::L => dest.write(&[0xBD]),
                Register8::A => dest.write(&[0xBF]),
            },
            CPInstruction::AHL => dest.write(&[0xBE]),
            CPInstruction::AN8(value) => dest.write(&[0xFE, *value]),
        }
    }
}

impl Display for CPInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CPInstruction::AR8(register8) => {
                f.write_fmt(format_args!("CP A, {}", register8.as_ref()))
            }
            CPInstruction::AHL => f.write_str("CP A, [HL]"),
            CPInstruction::AN8(value) => f.write_fmt(format_args!("CP A, 0x{:02X}", value)),
        }
    }
}

use std::{
    fmt::Display,
    io::Write,
    sync::{Arc, RwLock},
};

use crate::cpu::{
    CPU,
    asm::Assemble,
    cpu_bus::CPUBusTrait,
    instructions::InstructionBehavior,
    registers::{CPURegisters, Register8},
};

#[derive(Debug)]
pub enum ANDInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl InstructionBehavior for ANDInstruction {
    fn duration(&self) -> usize {
        match self {
            ANDInstruction::AR8(_) => 4,
            ANDInstruction::AHL => 8,
            ANDInstruction::AN8(_) => 8,
        }
    }
}

impl Assemble for ANDInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            ANDInstruction::AR8(register8) => match register8 {
                Register8::B => dest.write(&[0xA0]),
                Register8::C => dest.write(&[0xA1]),
                Register8::D => dest.write(&[0xA2]),
                Register8::E => dest.write(&[0xA3]),
                Register8::H => dest.write(&[0xA4]),
                Register8::L => dest.write(&[0xA5]),
                Register8::A => dest.write(&[0xA7]),
            },
            ANDInstruction::AHL => dest.write(&[0xA6]),
            ANDInstruction::AN8(value) => dest.write(&[0xE6, *value]),
        }
    }
}

impl Display for ANDInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ANDInstruction::AR8(register8) => {
                f.write_fmt(format_args!("AND A, {}", register8.as_ref()))
            }
            ANDInstruction::AHL => f.write_str("AND A, [HL]"),
            ANDInstruction::AN8(value) => f.write_fmt(format_args!("AND A, 0x{:02X}", value)),
        }
    }
}

#[derive(Debug)]
pub enum ORInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl ORInstruction {
    fn set_flags_after_or(&self, registers: &mut CPURegisters, result: u8) {
        registers.af.set_f(
            registers
                .af
                .f()
                .with_z(result == 0)
                .with_n(false)
                .with_h(false)
                .with_c(false),
        );
    }

    fn or_with_a_and_set_flags(&self, registers: &mut CPURegisters, value: u8) {
        let result = registers.af.a() | value;
        registers.af.set_a(result);
        self.set_flags_after_or(registers, result);
    }
}

impl InstructionBehavior for ORInstruction {
    fn duration(&self) -> usize {
        match self {
            ORInstruction::AR8(_) => 4,
            ORInstruction::AHL => 8,
            ORInstruction::AN8(_) => 8,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            ORInstruction::AR8(register8) => {
                let registers = &mut cpu.write().unwrap().registers;
                self.or_with_a_and_set_flags(registers, registers.get_r8(*register8));
            }
            ORInstruction::AHL => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                self.or_with_a_and_set_flags(&mut cpu.write().unwrap().registers, value);
            }
            ORInstruction::AN8(value) => {
                self.or_with_a_and_set_flags(&mut cpu.write().unwrap().registers, *value);
            }
        }
    }
}

impl Assemble for ORInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            ORInstruction::AR8(register8) => match register8 {
                Register8::B => dest.write(&[0xB0]),
                Register8::C => dest.write(&[0xB1]),
                Register8::D => dest.write(&[0xB2]),
                Register8::E => dest.write(&[0xB3]),
                Register8::H => dest.write(&[0xB4]),
                Register8::L => dest.write(&[0xB5]),
                Register8::A => dest.write(&[0xB7]),
            },
            ORInstruction::AHL => dest.write(&[0xB6]),
            ORInstruction::AN8(value) => dest.write(&[0xF6, *value]),
        }
    }
}

impl Display for ORInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ORInstruction::AR8(register8) => {
                f.write_fmt(format_args!("OR A, {}", register8.as_ref()))
            }
            ORInstruction::AHL => f.write_str("OR A, HL"),
            ORInstruction::AN8(value) => f.write_fmt(format_args!("OR A, 0x{:02X}", value)),
        }
    }
}

#[derive(Debug)]
pub enum XORInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl InstructionBehavior for XORInstruction {
    fn duration(&self) -> usize {
        match self {
            XORInstruction::AR8(_) => 4,
            XORInstruction::AHL => 8,
            XORInstruction::AN8(_) => 8,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            XORInstruction::AR8(register8) => {
                let registers = &mut cpu.write().unwrap().registers;
                registers
                    .af
                    .set_a(registers.af.a() ^ registers.get_r8(*register8));
            }
            XORInstruction::AHL => {
                let bus = cpu_bus;
                let value = bus.read_readonly(cpu.read().unwrap().registers.hl.into_bits());
                let registers = &mut cpu.write().unwrap().registers;
                registers.af.set_a(registers.af.a() ^ value)
            }
            XORInstruction::AN8(value) => {
                let registers = &mut cpu.write().unwrap().registers;
                registers.af.set_a(registers.af.a() ^ value);
            }
        }
    }
}

impl Assemble for XORInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            XORInstruction::AR8(register8) => match register8 {
                Register8::B => dest.write(&[0xA8]),
                Register8::C => dest.write(&[0xA9]),
                Register8::D => dest.write(&[0xAA]),
                Register8::E => dest.write(&[0xAB]),
                Register8::H => dest.write(&[0xAC]),
                Register8::L => dest.write(&[0xAD]),
                Register8::A => dest.write(&[0xAF]),
            },
            XORInstruction::AHL => dest.write(&[0xAE]),
            XORInstruction::AN8(value) => dest.write(&[0xEE, *value]),
        }
    }
}

impl Display for XORInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XORInstruction::AR8(register8) => {
                f.write_fmt(format_args!("XOR A, {}", register8.as_ref()))
            }
            XORInstruction::AHL => f.write_str("XOR A, HL"),
            XORInstruction::AN8(value) => f.write_fmt(format_args!("XOR A, 0x{:02X}", value)),
        }
    }
}

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
    registers::{CPURegisters, Register8, Register16},
};

#[derive(Debug)]
pub enum ADCInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl InstructionBehavior for ADCInstruction {
    fn duration(&self) -> usize {
        match self {
            ADCInstruction::AHL | ADCInstruction::AN8(_) => 8,
            _ => 4,
        }
    }
}

impl Assemble for ADCInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            ADCInstruction::AR8(from) => match from {
                Register8::A => dest.write(&[0x8F]),
                Register8::B => dest.write(&[0x88]),
                Register8::C => dest.write(&[0x89]),
                Register8::D => dest.write(&[0x8A]),
                Register8::E => dest.write(&[0x8B]),
                Register8::H => dest.write(&[0x8C]),
                Register8::L => dest.write(&[0x8D]),
            },
            ADCInstruction::AHL => dest.write(&[0x8E]),
            ADCInstruction::AN8(value) => dest.write(&[0xCE, *value]),
        }
    }
}

impl Display for ADCInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ADCInstruction::AR8(register8) => {
                f.write_fmt(format_args!("ADC A, {}", register8.as_ref()))
            }
            ADCInstruction::AHL => f.write_str("ADC A, [HL]"),
            ADCInstruction::AN8(value) => f.write_fmt(format_args!("ADC A, 0x{:02X}", value)),
        }
    }
}

#[derive(Debug)]
pub enum ADDInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
    SPE8(i8),
    HLR16(Register16),
    HLSP,
}

impl InstructionBehavior for ADDInstruction {
    fn duration(&self) -> usize {
        match self {
            ADDInstruction::HLR16(_) | ADDInstruction::HLSP | ADDInstruction::AHL => 8,
            ADDInstruction::AR8(_) => 4,
            ADDInstruction::AN8(_) => 8,
            ADDInstruction::SPE8(_) => 16,
        }
    }
}

impl Assemble for ADDInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            ADDInstruction::AR8(register8) => match register8 {
                Register8::B => dest.write(&[0x80]),
                Register8::C => dest.write(&[0x81]),
                Register8::D => dest.write(&[0x82]),
                Register8::E => dest.write(&[0x83]),
                Register8::H => dest.write(&[0x84]),
                Register8::L => dest.write(&[0x85]),
                Register8::A => dest.write(&[0x87]),
            },
            ADDInstruction::AHL => dest.write(&[0x86]),
            ADDInstruction::AN8(value) => dest.write(&[0xC6, *value]),
            ADDInstruction::SPE8(value) => dest.write(&[0xE8, (*value as u8)]),
            ADDInstruction::HLR16(register16) => match register16 {
                Register16::BC => dest.write(&[0x09]),
                Register16::DE => dest.write(&[0x19]),
                Register16::HL => dest.write(&[0x29]),
            },
            ADDInstruction::HLSP => dest.write(&[0x39]),
        }
    }
}

impl Display for ADDInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ADDInstruction::AR8(register8) => {
                f.write_fmt(format_args!("ADD A, {}", register8.as_ref()))
            }
            ADDInstruction::AHL => f.write_str("ADD A, [HL]"),
            ADDInstruction::AN8(value) => f.write_fmt(format_args!("ADD A, 0x{:02X}", value)),
            ADDInstruction::SPE8(offset) => f.write_fmt(format_args!("ADD SP, {}", offset)),
            ADDInstruction::HLR16(register16) => {
                f.write_fmt(format_args!("ADD HL, {}", register16.as_ref()))
            }
            ADDInstruction::HLSP => f.write_str("ADD HL, SP"),
        }
    }
}

#[derive(Debug)]
pub enum DAAInstruction {
    Empty,
}

impl InstructionBehavior for DAAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for DAAInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x27])
    }
}

impl Display for DAAInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DAA")
    }
}

#[derive(Debug)]
pub enum DECInstruction {
    R8(Register8),
    HL,
    R16(Register16),
    SP,
}

impl DECInstruction {
    fn set_flags_after_dec(&self, registers: &mut CPURegisters, new_value: u8) {
        registers.af.set_f(
            registers
                .af
                .f()
                .with_z(new_value == 0)
                .with_n(true)
                .with_h(new_value & 0x0F == 0x0F),
        );
    }
}

impl InstructionBehavior for DECInstruction {
    fn duration(&self) -> usize {
        match self {
            DECInstruction::R8(_) => 4,
            DECInstruction::HL => 12,
            DECInstruction::R16(_) => 8,
            DECInstruction::SP => 8,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            DECInstruction::R8(register8) => {
                let registers = &mut cpu.write().unwrap().registers;
                let value = registers.get_r8(*register8);
                let new_value = value.wrapping_sub(1);
                registers.set_r8(*register8, new_value);
                self.set_flags_after_dec(registers, new_value);
            }
            DECInstruction::HL => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                let new_value = value.wrapping_sub(1);
                cpu_bus.write(addr, new_value);
                self.set_flags_after_dec(&mut cpu.write().unwrap().registers, new_value);
            }
            DECInstruction::R16(register16) => {
                let registers = &mut cpu.write().unwrap().registers;
                let value = registers.get_r16(*register16);
                registers.set_r16(*register16, value.wrapping_sub(1));
            }
            DECInstruction::SP => {
                let registers = &mut cpu.write().unwrap().registers;
                registers.sp = registers.sp.wrapping_sub(1);
            }
        }
    }
}

impl Assemble for DECInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            DECInstruction::R8(register8) => match register8 {
                Register8::B => dest.write(&[0x05]),
                Register8::C => dest.write(&[0x0D]),
                Register8::D => dest.write(&[0x15]),
                Register8::E => dest.write(&[0x1D]),
                Register8::H => dest.write(&[0x25]),
                Register8::L => dest.write(&[0x2D]),
                Register8::A => dest.write(&[0x3D]),
            },
            DECInstruction::HL => dest.write(&[0x35]),
            DECInstruction::R16(register16) => match register16 {
                Register16::BC => dest.write(&[0x0B]),
                Register16::DE => dest.write(&[0x1B]),
                Register16::HL => dest.write(&[0x2B]),
            },
            DECInstruction::SP => dest.write(&[0x3B]),
        }
    }
}

impl Display for DECInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DECInstruction::R8(register8) => {
                f.write_fmt(format_args!("DEC {}", register8.as_ref()))
            }
            DECInstruction::HL => f.write_str("DEC HL"),
            DECInstruction::R16(register16) => {
                f.write_fmt(format_args!("DEC {}", register16.as_ref()))
            }
            DECInstruction::SP => f.write_str("DEC SP"),
        }
    }
}

#[derive(Debug)]
pub enum INCInstruction {
    R8(Register8),
    HL,
    R16(Register16),
    SP,
}

impl INCInstruction {
    fn set_flags_after_inc(&self, registers: &mut CPURegisters, new_value: u8) {
        registers.af.set_f(
            registers
                .af
                .f()
                .with_z(new_value == 0)
                .with_n(false)
                .with_h(new_value & 0x0F == 0),
        );
    }
}

impl InstructionBehavior for INCInstruction {
    fn duration(&self) -> usize {
        match self {
            INCInstruction::R8(_) => 4,
            INCInstruction::HL => 12,
            INCInstruction::R16(_) => 8,
            INCInstruction::SP => 8,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            INCInstruction::R8(register8) => {
                let registers = &mut cpu.write().unwrap().registers;
                let value = registers.get_r8(*register8);
                let new_value = value.wrapping_add(1);
                registers.set_r8(*register8, new_value);
                self.set_flags_after_inc(registers, new_value);
            }
            INCInstruction::HL => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                let new_value = value.wrapping_add(1);
                cpu_bus.write(addr, new_value);
                self.set_flags_after_inc(&mut cpu.write().unwrap().registers, new_value);
            }
            INCInstruction::R16(register16) => {
                let registers = &mut cpu.write().unwrap().registers;
                let value = registers.get_r16(*register16);
                registers.set_r16(*register16, value.wrapping_add(1));
            }
            INCInstruction::SP => {
                let registers = &mut cpu.write().unwrap().registers;
                registers.sp = registers.sp.wrapping_add(1);
            }
        }
    }
}

impl Assemble for INCInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            INCInstruction::R8(register8) => match register8 {
                Register8::B => dest.write(&[0x04]),
                Register8::C => dest.write(&[0x0C]),
                Register8::D => dest.write(&[0x14]),
                Register8::E => dest.write(&[0x1C]),
                Register8::H => dest.write(&[0x24]),
                Register8::L => dest.write(&[0x2C]),
                Register8::A => dest.write(&[0x3C]),
            },
            INCInstruction::HL => dest.write(&[0x34]),
            INCInstruction::R16(register16) => match register16 {
                Register16::BC => dest.write(&[0x03]),
                Register16::DE => dest.write(&[0x13]),
                Register16::HL => dest.write(&[0x23]),
            },
            INCInstruction::SP => dest.write(&[0x33]),
        }
    }
}

impl Display for INCInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            INCInstruction::R8(register8) => {
                f.write_fmt(format_args!("INC {}", register8.as_ref()))
            }
            INCInstruction::HL => f.write_str("INC HL"),
            INCInstruction::R16(register16) => {
                f.write_fmt(format_args!("INC {}", register16.as_ref()))
            }
            INCInstruction::SP => f.write_str("INC SP"),
        }
    }
}

#[derive(Debug)]
pub enum SBCInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl InstructionBehavior for SBCInstruction {
    fn duration(&self) -> usize {
        todo!()
    }
}

impl Assemble for SBCInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            SBCInstruction::AR8(register8) => match register8 {
                Register8::B => dest.write(&[0x98]),
                Register8::C => dest.write(&[0x99]),
                Register8::D => dest.write(&[0x9A]),
                Register8::E => dest.write(&[0x9B]),
                Register8::H => dest.write(&[0x9C]),
                Register8::L => dest.write(&[0x9D]),
                Register8::A => dest.write(&[0x9F]),
            },
            &SBCInstruction::AHL => dest.write(&[0x9E]),
            SBCInstruction::AN8(value) => dest.write(&[0xDA, *value]),
        }
    }
}

impl Display for SBCInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SBCInstruction::AR8(register8) => {
                f.write_fmt(format_args!("SBC A, {}", register8.as_ref()))
            }
            SBCInstruction::AHL => f.write_str("SBC A, [HL]"),
            SBCInstruction::AN8(value) => f.write_fmt(format_args!("SBC A, 0x{:02X}", value)),
        }
    }
}

#[derive(Debug)]
pub enum SUBInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl InstructionBehavior for SUBInstruction {
    fn duration(&self) -> usize {
        match self {
            SUBInstruction::AR8(_) => 4,
            SUBInstruction::AHL => 8,
            SUBInstruction::AN8(_) => 8,
        }
    }
}

impl Assemble for SUBInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            SUBInstruction::AR8(register8) => match register8 {
                Register8::B => dest.write(&[0x90]),
                Register8::C => dest.write(&[0x91]),
                Register8::D => dest.write(&[0x92]),
                Register8::E => dest.write(&[0x93]),
                Register8::H => dest.write(&[0x94]),
                Register8::L => dest.write(&[0x95]),
                Register8::A => dest.write(&[0x97]),
            },
            SUBInstruction::AHL => dest.write(&[0x96]),
            SUBInstruction::AN8(value) => dest.write(&[0xD6, *value]),
        }
    }
}

impl Display for SUBInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SUBInstruction::AR8(register8) => {
                f.write_fmt(format_args!("SUB A, {}", register8.as_ref()))
            }
            SUBInstruction::AHL => f.write_str("SUB A, [HL]"),
            SUBInstruction::AN8(value) => f.write_fmt(format_args!("SUB A, 0x{:02X}", value)),
        }
    }
}

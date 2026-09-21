use std::{
    fmt::Display,
    io::Write,
    sync::{Arc, RwLock},
};

use bytemuck::bytes_of;
use zendian::le::u16le;

use crate::cpu::{
    CPU,
    cpu_bus::CPUBusTrait,
    instructions::{Assemble, InstructionBehavior},
    registers::{HLRegister, Register8, Register16},
};

#[derive(Debug)]
pub enum LDInstruction {
    R8R8(Register8, Register8),
    R8N8(Register8, u8),
    R16N16(Register16, u16),
    SPN16(u16),
    N16SP(u16),
    HLR8(Register8),
    HLN8(u8),
    R8HL(Register8),
    R16A(Register16),
    N16A(u16),
    AR16(Register16),
    AN16(u16),
    HLIA,
    HLDA,
    AHLI,
    AHLD,
    HLSPE8(i8),
    SPHL,
}

impl InstructionBehavior for LDInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            LDInstruction::R8R8(to, from) => {
                let registers = &mut cpu.write().unwrap().registers;
                let value = registers.get_r8(*from);
                registers.set_r8(*to, value);
            }
            LDInstruction::R8N8(to, value) => cpu.write().unwrap().registers.set_r8(*to, *value),
            LDInstruction::R16N16(to, value) => cpu.write().unwrap().registers.set_r16(*to, *value),
            LDInstruction::SPN16(value) => cpu.write().unwrap().registers.sp = *value,
            LDInstruction::N16SP(addr) => {
                let sp = cpu.read().unwrap().registers.sp;
                cpu_bus.write(*addr, (sp & 0xFF) as u8);
                cpu_bus.write(*addr + 1, (sp >> 8) as u8);
            }
            LDInstruction::HLR8(from) => {
                let to = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu.read().unwrap().registers.get_r8(*from);
                cpu_bus.write(to, value);
            }
            LDInstruction::HLN8(value) => {
                let to = cpu.read().unwrap().registers.hl.into_bits();
                cpu_bus.write(to, *value);
            }
            LDInstruction::R8HL(to) => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                cpu.write().unwrap().registers.set_r8(*to, value);
            }
            LDInstruction::R16A(to) => {
                let addr = cpu.read().unwrap().registers.get_r16(*to);
                let value = cpu.read().unwrap().registers.af.a();
                cpu_bus.write(addr, value);
            }
            LDInstruction::N16A(addr) => {
                let value = cpu.read().unwrap().registers.af.a();
                cpu_bus.write(*addr, value);
            }
            LDInstruction::AR16(register16) => {
                let addr = cpu.read().unwrap().registers.get_r16(*register16);
                let value = cpu_bus.read(addr);
                cpu.write().unwrap().registers.af.set_a(value);
            }
            LDInstruction::AN16(addr) => {
                let value = cpu_bus.read(*addr);
                cpu.write().unwrap().registers.af.set_a(value);
            }
            LDInstruction::HLIA => {
                let to = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu.read().unwrap().registers.af.a();
                cpu_bus.write(to, value);
                cpu.write().unwrap().registers.hl = HLRegister::from_bits(to.wrapping_add(1));
            }
            LDInstruction::HLDA => {
                let to = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu.read().unwrap().registers.af.a();
                cpu_bus.write(to, value);
                cpu.write().unwrap().registers.hl = HLRegister::from_bits(to.wrapping_sub(1));
            }
            LDInstruction::AHLI => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                let registers = &mut cpu.write().unwrap().registers;
                registers.af.set_a(value);
                registers.hl = HLRegister::from_bits(addr.wrapping_add(1));
            }
            LDInstruction::AHLD => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                let registers = &mut cpu.write().unwrap().registers;
                registers.af.set_a(value);
                registers.hl = HLRegister::from_bits(addr.wrapping_sub(1));
            }
            LDInstruction::HLSPE8(_) => todo!(),
            LDInstruction::SPHL => todo!(),
        }
    }

    fn duration(&self) -> usize {
        match self {
            LDInstruction::R8R8(_, _) => 4,
            LDInstruction::R8N8(_, _) => 8,
            LDInstruction::R16N16(_, _) => 12,
            LDInstruction::SPN16(_) => 12,
            LDInstruction::N16SP(_) => 20,
            LDInstruction::HLR8(_) => 8,
            LDInstruction::HLN8(_) => 12,
            LDInstruction::R8HL(_) => 8,
            LDInstruction::R16A(_) => 8,
            LDInstruction::N16A(_) => 16,
            LDInstruction::AR16(_) => 8,
            LDInstruction::AN16(_) => 16,
            LDInstruction::HLIA => 8,
            LDInstruction::HLDA => 8,
            LDInstruction::AHLI => 8,
            LDInstruction::AHLD => 8,
            LDInstruction::HLSPE8(_) => 12,
            LDInstruction::SPHL => 8,
        }
    }
}

impl Display for LDInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LDInstruction::R8R8(to, from) => {
                f.write_fmt(format_args!("LD {}, {}", to.as_ref(), from.as_ref()))
            }
            LDInstruction::R8N8(to, value) => {
                f.write_fmt(format_args!("LD {}, 0x{:02X}", to.as_ref(), value))
            }
            LDInstruction::R16N16(to, value) => {
                f.write_fmt(format_args!("LD {}, 0x{:04X}", to.as_ref(), value))
            }
            LDInstruction::SPN16(value) => f.write_fmt(format_args!("LD SP, 0x{:04X}", value)),
            LDInstruction::N16SP(to) => f.write_fmt(format_args!("LD [${:04X}], SP", to)),
            LDInstruction::HLR8(from) => f.write_fmt(format_args!("LD [HL], {}", from.as_ref())),
            LDInstruction::HLN8(value) => f.write_fmt(format_args!("LD [HL], 0x{:02X}", value)),
            LDInstruction::R8HL(to) => f.write_fmt(format_args!("LD {}, [HL]", to.as_ref())),
            LDInstruction::R16A(register16) => {
                f.write_fmt(format_args!("LD [{}], A", register16.as_ref()))
            }
            LDInstruction::N16A(to) => f.write_fmt(format_args!("LD [${:04X}], A", to)),
            LDInstruction::AR16(register16) => {
                f.write_fmt(format_args!("LD A, [{}]", register16.as_ref()))
            }
            LDInstruction::AN16(from) => f.write_fmt(format_args!("LD A, [${:04X}]", from)),
            LDInstruction::HLIA => f.write_str("LD [HL+], A"),
            LDInstruction::HLDA => f.write_str("LD [HL-], A"),
            LDInstruction::AHLI => f.write_str("LD A, [HL+]"),
            LDInstruction::AHLD => f.write_str("LD A, [HL-]"),
            LDInstruction::HLSPE8(offset) => f.write_fmt(format_args!(
                "LD HL, SP {} {}",
                if *offset < 0 { "-" } else { "+" },
                offset.abs(),
            )),
            LDInstruction::SPHL => f.write_str("LD SP, HL"),
        }
    }
}

impl Assemble for LDInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            LDInstruction::R8R8(to, from) => match (to, from) {
                (Register8::B, Register8::B) => dest.write(&[0x40]),
                (Register8::B, Register8::C) => dest.write(&[0x41]),
                (Register8::B, Register8::D) => dest.write(&[0x42]),
                (Register8::B, Register8::E) => dest.write(&[0x43]),
                (Register8::B, Register8::H) => dest.write(&[0x44]),
                (Register8::B, Register8::L) => dest.write(&[0x45]),
                (Register8::B, Register8::A) => dest.write(&[0x47]),

                (Register8::C, Register8::B) => dest.write(&[0x48]),
                (Register8::C, Register8::C) => dest.write(&[0x49]),
                (Register8::C, Register8::D) => dest.write(&[0x4A]),
                (Register8::C, Register8::E) => dest.write(&[0x4B]),
                (Register8::C, Register8::H) => dest.write(&[0x4C]),
                (Register8::C, Register8::L) => dest.write(&[0x4D]),
                (Register8::C, Register8::A) => dest.write(&[0x4F]),

                (Register8::D, Register8::B) => dest.write(&[0x50]),
                (Register8::D, Register8::C) => dest.write(&[0x51]),
                (Register8::D, Register8::D) => dest.write(&[0x52]),
                (Register8::D, Register8::E) => dest.write(&[0x53]),
                (Register8::D, Register8::H) => dest.write(&[0x54]),
                (Register8::D, Register8::L) => dest.write(&[0x55]),
                (Register8::D, Register8::A) => dest.write(&[0x57]),

                (Register8::E, Register8::B) => dest.write(&[0x58]),
                (Register8::E, Register8::C) => dest.write(&[0x59]),
                (Register8::E, Register8::D) => dest.write(&[0x5A]),
                (Register8::E, Register8::E) => dest.write(&[0x5B]),
                (Register8::E, Register8::H) => dest.write(&[0x5C]),
                (Register8::E, Register8::L) => dest.write(&[0x5D]),
                (Register8::E, Register8::A) => dest.write(&[0x5F]),

                (Register8::H, Register8::B) => dest.write(&[0x60]),
                (Register8::H, Register8::C) => dest.write(&[0x61]),
                (Register8::H, Register8::D) => dest.write(&[0x62]),
                (Register8::H, Register8::E) => dest.write(&[0x63]),
                (Register8::H, Register8::H) => dest.write(&[0x64]),
                (Register8::H, Register8::L) => dest.write(&[0x65]),
                (Register8::H, Register8::A) => dest.write(&[0x67]),

                (Register8::L, Register8::B) => dest.write(&[0x68]),
                (Register8::L, Register8::C) => dest.write(&[0x69]),
                (Register8::L, Register8::D) => dest.write(&[0x6A]),
                (Register8::L, Register8::E) => dest.write(&[0x6B]),
                (Register8::L, Register8::H) => dest.write(&[0x6C]),
                (Register8::L, Register8::L) => dest.write(&[0x6D]),
                (Register8::L, Register8::A) => dest.write(&[0x6F]),

                (Register8::A, Register8::B) => dest.write(&[0x78]),
                (Register8::A, Register8::C) => dest.write(&[0x79]),
                (Register8::A, Register8::D) => dest.write(&[0x7A]),
                (Register8::A, Register8::E) => dest.write(&[0x7B]),
                (Register8::A, Register8::H) => dest.write(&[0x7C]),
                (Register8::A, Register8::L) => dest.write(&[0x7D]),
                (Register8::A, Register8::A) => dest.write(&[0x7F]),
            },
            LDInstruction::R8N8(to, value) => match to {
                Register8::B => dest.write(&[0x06, *value]),
                Register8::C => dest.write(&[0x0E, *value]),
                Register8::D => dest.write(&[0x16, *value]),
                Register8::E => dest.write(&[0x1E, *value]),
                Register8::H => dest.write(&[0x26, *value]),
                Register8::L => dest.write(&[0x2E, *value]),
                Register8::A => dest.write(&[0x3E, *value]),
            },
            LDInstruction::R16N16(register16, value) => match register16 {
                Register16::BC => {
                    Ok(dest.write(&[0x01])? + dest.write(bytes_of(&u16le::from(value)))?)
                }
                Register16::DE => {
                    Ok(dest.write(&[0x11])? + dest.write(bytes_of(&u16le::from(value)))?)
                }
                Register16::HL => {
                    Ok(dest.write(&[0x12])? + dest.write(bytes_of(&u16le::from(value)))?)
                }
            },
            LDInstruction::SPN16(value) => {
                Ok(dest.write(&[0x31])? + dest.write(bytes_of(&u16le::from(value)))?)
            }
            LDInstruction::N16SP(value) => {
                Ok(dest.write(&[0x08])? + dest.write(bytes_of(&u16le::from(value)))?)
            }
            LDInstruction::HLR8(from) => match from {
                Register8::B => dest.write(&[0x70]),
                Register8::C => dest.write(&[0x71]),
                Register8::D => dest.write(&[0x72]),
                Register8::E => dest.write(&[0x73]),
                Register8::H => dest.write(&[0x74]),
                Register8::L => dest.write(&[0x75]),
                Register8::A => dest.write(&[0x77]),
            },
            LDInstruction::HLN8(value) => dest.write(&[0x36, *value]),
            LDInstruction::R8HL(to) => match to {
                Register8::B => dest.write(&[0x46]),
                Register8::C => dest.write(&[0x4E]),
                Register8::D => dest.write(&[0x56]),
                Register8::E => dest.write(&[0x5E]),
                Register8::H => dest.write(&[0x66]),
                Register8::L => dest.write(&[0x6E]),
                Register8::A => dest.write(&[0x7E]),
            },
            LDInstruction::R16A(to) => match to {
                Register16::BC => dest.write(&[0x02]),
                Register16::DE => dest.write(&[0x12]),
                Register16::HL => dest.write(&[0x77]),
            },
            LDInstruction::N16A(value) => {
                Ok(dest.write(&[0xEA])? + dest.write(bytes_of(&u16le::from(value)))?)
            }
            LDInstruction::AR16(from) => match from {
                Register16::BC => dest.write(&[0x0A]),
                Register16::DE => dest.write(&[0x1A]),
                Register16::HL => dest.write(&[0x7E]),
            },
            LDInstruction::AN16(value) => {
                Ok(dest.write(&[0xFA])? + dest.write(bytes_of(&u16le::from(value)))?)
            }
            LDInstruction::HLIA => dest.write(&[0x22]),
            LDInstruction::HLDA => dest.write(&[0x32]),
            LDInstruction::AHLI => dest.write(&[0x2A]),
            LDInstruction::AHLD => dest.write(&[0x3A]),
            LDInstruction::HLSPE8(offset) => dest.write(&[0xF8, (*offset as u8)]),
            LDInstruction::SPHL => dest.write(&[0xF9]),
        }
    }
}

#[derive(Debug)]
pub enum LDHInstruction {
    N8A(u8),
    CA,
    AN8(u8),
    AC,
}

impl InstructionBehavior for LDHInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            LDHInstruction::N8A(offset) => {
                let value = cpu.read().unwrap().registers.af.a();
                cpu_bus.write(0xFF00 + (*offset as u16), value);
            }
            LDHInstruction::CA => {
                let offset = cpu.read().unwrap().registers.bc.c();
                let value = cpu.read().unwrap().registers.af.a();
                cpu_bus.write(0xFF00 + (offset as u16), value);
            }
            LDHInstruction::AN8(offset) => {
                let value = cpu_bus.read(0xFF00 + (*offset as u16));
                cpu.write().unwrap().registers.af.set_a(value);
            }
            LDHInstruction::AC => {
                let offset = cpu.read().unwrap().registers.bc.c();
                let value = cpu_bus.read(0xFF00 + (offset as u16));
                cpu.write().unwrap().registers.af.set_a(value);
            }
        }
    }

    fn duration(&self) -> usize {
        match self {
            LDHInstruction::N8A(_) => 12,
            LDHInstruction::CA => 8,
            LDHInstruction::AN8(_) => 12,
            LDHInstruction::AC => 8,
        }
    }
}

impl Assemble for LDHInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            LDHInstruction::N8A(high_byte) => dest.write(&[0xE0, *high_byte]),
            LDHInstruction::CA => dest.write(&[0xE2]),
            LDHInstruction::AN8(high_byte) => dest.write(&[0xF0, *high_byte]),
            LDHInstruction::AC => dest.write(&[0xF2]),
        }
    }
}

impl Display for LDHInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LDHInstruction::N8A(offset) => f.write_fmt(format_args!("LDH A, [0x{:02X}]", offset)),
            LDHInstruction::CA => f.write_str("LDH [C], A"),
            LDHInstruction::AN8(offset) => f.write_fmt(format_args!("LDH [0x{:02X}], A", offset)),
            LDHInstruction::AC => f.write_str("LDH A, [C]"),
        }
    }
}

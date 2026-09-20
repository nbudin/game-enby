use std::{
    fmt::{Debug, Display},
    io::Read,
};

use bytemuck::checked::from_bytes;
use zendian::le::u16le;

use crate::cpu::{
    instructions::*,
    operand::ConditionCode,
    registers::{Register8, Register16},
};

fn read_u8(src: &mut impl Read) -> Result<u8, std::io::Error> {
    let mut buf = [0u8];
    src.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn read_i8(src: &mut impl Read) -> Result<i8, std::io::Error> {
    let byte = read_u8(src)?;
    Ok(byte as i8)
}

fn read_u16le(src: &mut impl Read) -> Result<u16, std::io::Error> {
    let mut buf = [0u8; 2];
    src.read_exact(&mut buf)?;
    Ok(u16le::from_bits(*from_bytes(&buf)).bits())
}

pub fn read_instruction(src: &mut impl Read) -> Result<Instruction, std::io::Error> {
    let opcode = read_u8(src)?;

    Ok(match opcode {
        0x00 => NOPInstruction::Empty.into(),
        0x01 => LDInstruction::R16N16(Register16::BC, read_u16le(src)?).into(),
        0x02 => LDInstruction::R16A(Register16::BC).into(),
        0x03 => INCInstruction::R16(Register16::BC).into(),
        0x04 => INCInstruction::R8(Register8::B).into(),
        0x05 => DECInstruction::R8(Register8::B).into(),
        0x06 => LDInstruction::R8N8(Register8::B, read_u8(src)?).into(),
        0x07 => RLCAInstruction::Empty.into(),
        0x08 => LDInstruction::N16SP(read_u16le(src)?).into(),
        0x09 => ADDInstruction::HLR16(Register16::BC).into(),
        0x0A => LDInstruction::AR16(Register16::BC).into(),
        0x0B => DECInstruction::R16(Register16::BC).into(),
        0x0C => INCInstruction::R8(Register8::C).into(),
        0x0D => DECInstruction::R8(Register8::C).into(),
        0x0E => LDInstruction::R8N8(Register8::C, read_u8(src)?).into(),
        0x0F => RRCAInstruction::Empty.into(),

        0x10 => {
            let _ = read_u8(src)?;
            STOPInstruction::Empty.into()
        }
        0x11 => LDInstruction::R16N16(Register16::DE, read_u16le(src)?).into(),
        0x12 => LDInstruction::R16A(Register16::DE).into(),
        0x13 => INCInstruction::R16(Register16::DE).into(),
        0x14 => INCInstruction::R8(Register8::D).into(),
        0x15 => DECInstruction::R8(Register8::D).into(),
        0x16 => LDInstruction::R8N8(Register8::D, read_u8(src)?).into(),
        0x17 => RLAInstruction::Empty.into(),

        0x20 => JRInstruction::CCE8(ConditionCode::NZ, read_i8(src)?).into(),
        0x21 => LDInstruction::R16N16(Register16::HL, read_u16le(src)?).into(),
        0x2A => LDInstruction::AHLI.into(),

        0x31 => LDInstruction::SPN16(read_u16le(src)?).into(),
        0x32 => LDInstruction::HLDA.into(),
        0x36 => LDInstruction::HLN8(read_u8(src)?).into(),
        0x3A => LDInstruction::AHLD.into(),
        0x3E => LDInstruction::R8N8(Register8::A, read_u8(src)?).into(),

        0x80 => ADDInstruction::AR8(Register8::B).into(),
        0x81 => ADDInstruction::AR8(Register8::C).into(),
        0x82 => ADDInstruction::AR8(Register8::D).into(),
        0x83 => ADDInstruction::AR8(Register8::E).into(),
        0x84 => ADDInstruction::AR8(Register8::H).into(),
        0x85 => ADDInstruction::AR8(Register8::L).into(),
        0x86 => ADDInstruction::AHL.into(),
        0x87 => ADDInstruction::AR8(Register8::A).into(),
        0x88 => ADCInstruction::AR8(Register8::B).into(),
        0x89 => ADCInstruction::AR8(Register8::C).into(),
        0x8A => ADCInstruction::AR8(Register8::D).into(),
        0x8B => ADCInstruction::AR8(Register8::E).into(),
        0x8C => ADCInstruction::AR8(Register8::H).into(),
        0x8D => ADCInstruction::AR8(Register8::L).into(),
        0x8E => ADCInstruction::AHL.into(),
        0x8F => ADCInstruction::AR8(Register8::A).into(),

        0xAF => XORInstruction::AR8(Register8::A).into(),

        0xC3 => JPInstruction::N16(read_u16le(src)?).into(),

        0xE0 => LDHInstruction::N8A(read_u8(src)?).into(),
        0xE2 => LDHInstruction::CA.into(),
        0xEA => LDInstruction::N16A(read_u16le(src)?).into(),

        0xF0 => LDHInstruction::AN8(read_u8(src)?).into(),
        0xF2 => LDHInstruction::AC.into(),
        0xF3 => CCFInstruction::Empty.into(),
        0xFE => CPInstruction::AN8(read_u8(src)?).into(),

        _ => todo!("Unknown opcode: {:02X}", opcode),
    })
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::CCFInstruction(_) => f.write_str("CCF"),
            Instruction::CPInstruction(instruction) => Display::fmt(&instruction, f),
            Instruction::DECInstruction(instruction) => Display::fmt(&instruction, f),
            Instruction::INCInstruction(instruction) => Display::fmt(&instruction, f),
            Instruction::LDInstruction(instruction) => Display::fmt(&instruction, f),
            Instruction::LDHInstruction(instruction) => Display::fmt(&instruction, f),
            Instruction::JPInstruction(instruction) => Display::fmt(&instruction, f),
            Instruction::JRInstruction(instruction) => Display::fmt(&instruction, f),
            Instruction::NOPInstruction(_) => f.write_str("NOP"),
            Instruction::XORInstruction(instruction) => Display::fmt(&instruction, f),
            _ => todo!("{:?}", self),
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
impl Display for JRInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JRInstruction::E8(offset) => f.write_fmt(format_args!("JR {}", offset)),
            JRInstruction::CCE8(condition_code, offset) => {
                f.write_fmt(format_args!("JR {}, {}", condition_code.as_ref(), offset))
            }
        }
    }
}

impl Display for LDInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LDInstruction::R8R8(to, from) => todo!(),
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
            LDInstruction::R16A(register16) => todo!(),
            LDInstruction::N16A(to) => f.write_fmt(format_args!("LD [${:04X}], A", to)),
            LDInstruction::AR16(register16) => todo!(),
            LDInstruction::AN16(from) => f.write_fmt(format_args!("LD A, [${:04X}]", from)),
            LDInstruction::HLIA => f.write_str("LD [HL+], A"),
            LDInstruction::HLDA => f.write_str("LD [HL-], A"),
            LDInstruction::AHLI => f.write_str("LD A, [HL+]"),
            LDInstruction::AHLD => f.write_str("LD A, [HL-]"),
            LDInstruction::HLSPE8(_) => todo!(),
            LDInstruction::SPHL => todo!(),
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

impl Display for JPInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JPInstruction::HL => f.write_str("JP HL"),
            JPInstruction::N16(addr) => f.write_fmt(format_args!("JP ${:04X}", addr)),
            JPInstruction::CCN16(condition_code, addr) => {
                f.write_fmt(format_args!("JP {},${:04X}", condition_code.as_ref(), addr))
            }
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

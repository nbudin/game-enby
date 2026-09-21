use std::{
    fmt::{Debug, Display},
    io::Read,
};

use bytemuck::checked::from_bytes;
use zendian::le::u16le;

use crate::cpu::{
    instructions::{
        arithmetic::{ADCInstruction, ADDInstruction, DECInstruction, INCInstruction},
        bitwise::{RLAInstruction, RLCAInstruction, RRCAInstruction},
        control::{
            CALLInstruction, JPInstruction, JRInstruction, NOPInstruction, RETIInstruction,
            RETInstruction, STOPInstruction,
        },
        flags::CCFInstruction,
        load::{LDHInstruction, LDInstruction},
        logic::{ORInstruction, XORInstruction},
        test::CPInstruction,
        *,
    },
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

        0x78 => LDInstruction::R8R8(Register8::A, Register8::B).into(),
        0x79 => LDInstruction::R8R8(Register8::A, Register8::C).into(),
        0x7A => LDInstruction::R8R8(Register8::A, Register8::D).into(),
        0x7B => LDInstruction::R8R8(Register8::A, Register8::E).into(),
        0x7C => LDInstruction::R8R8(Register8::A, Register8::H).into(),
        0x7D => LDInstruction::R8R8(Register8::A, Register8::L).into(),
        0x7E => LDInstruction::R8HL(Register8::A).into(),
        0x7F => LDInstruction::R8R8(Register8::A, Register8::A).into(),

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

        0xB0 => ORInstruction::AR8(Register8::B).into(),
        0xB1 => ORInstruction::AR8(Register8::C).into(),
        0xB2 => ORInstruction::AR8(Register8::D).into(),
        0xB3 => ORInstruction::AR8(Register8::E).into(),
        0xB4 => ORInstruction::AR8(Register8::H).into(),
        0xB5 => ORInstruction::AR8(Register8::L).into(),
        0xB6 => ORInstruction::AHL.into(),
        0xB7 => ORInstruction::AR8(Register8::A).into(),

        0xC0 => RETInstruction::Conditional(ConditionCode::NZ).into(),
        0xC3 => JPInstruction::N16(read_u16le(src)?).into(),
        0xC4 => CALLInstruction::CCN16(ConditionCode::NZ, read_u16le(src)?).into(),
        0xCC => CALLInstruction::CCN16(ConditionCode::Z, read_u16le(src)?).into(),
        0xCD => CALLInstruction::N16(read_u16le(src)?).into(),
        0xC8 => RETInstruction::Conditional(ConditionCode::Z).into(),
        0xC9 => RETInstruction::Unconditional.into(),

        0xD0 => RETInstruction::Conditional(ConditionCode::NC).into(),
        0xD4 => CALLInstruction::CCN16(ConditionCode::NC, read_u16le(src)?).into(),
        0xD8 => RETInstruction::Conditional(ConditionCode::C).into(),
        0xD9 => RETIInstruction::Empty.into(),
        0xDC => CALLInstruction::CCN16(ConditionCode::C, read_u16le(src)?).into(),

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

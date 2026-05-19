use std::io::Read;

use bytemuck::checked::from_bytes;
use zendian::le::u16le;

use crate::cpu::{
    instructions::*,
    registers::{Register8, Register16},
};

fn read_u8(src: &mut impl Read) -> Result<u8, std::io::Error> {
    let mut buf = [0u8];
    src.read_exact(&mut buf)?;
    Ok(buf[0])
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

        0x3E => LDInstruction::R8N8(Register8::A, read_u8(src)?).into(),

        0xC3 => JPInstruction::N16(read_u16le(src)?).into(),

        0xE0 => LDHInstruction::N8A(read_u8(src)?).into(),

        0xF3 => CCFInstruction::Empty.into(),

        _ => todo!("Unknown opcode: {:02X}", opcode),
    })
}

use std::io::Write;

use bytemuck::bytes_of;
use strum::FromRepr;
use zendian::le::u16le;

use crate::cpu::registers::{Register8, Register16};

#[derive(Debug, FromRepr, Copy, Clone)]
#[repr(u8)]
enum BitOffset {
    Bit0 = 0,
    Bit1 = 1,
    Bit2 = 2,
    Bit3 = 3,
    Bit4 = 4,
    Bit5 = 5,
    Bit6 = 6,
    Bit7 = 7,
}

pub trait Instruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error>;
    fn duration(&self) -> usize;
}

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

impl Instruction for LDInstruction {
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

#[derive(Debug)]
pub enum LDHInstruction {
    N8A(u8),
    CA,
    AN8(u8),
    AC,
}

impl Instruction for LDHInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            LDHInstruction::N8A(high_byte) => dest.write(&[0xE0, *high_byte]),
            LDHInstruction::CA => dest.write(&[0xE2]),
            LDHInstruction::AN8(high_byte) => dest.write(&[0xF0, *high_byte]),
            LDHInstruction::AC => dest.write(&[0xF2]),
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

#[derive(Debug)]
pub enum ADCInstruction {
    AA,
    AB,
    AC,
    AD,
    AE,
    AH,
    AL,
    AHL,
    An8(u8),
}

impl Instruction for ADCInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            ADCInstruction::AA => dest.write(&[0x8F]),
            ADCInstruction::AB => dest.write(&[0x88]),
            ADCInstruction::AC => dest.write(&[0x89]),
            ADCInstruction::AD => dest.write(&[0x8A]),
            ADCInstruction::AE => dest.write(&[0x8B]),
            ADCInstruction::AH => dest.write(&[0x8C]),
            ADCInstruction::AL => dest.write(&[0x8D]),
            ADCInstruction::AHL => dest.write(&[0x8E]),
            ADCInstruction::An8(value) => dest.write(&[0xCE, *value]),
        }
    }

    fn duration(&self) -> usize {
        match self {
            ADCInstruction::AHL | ADCInstruction::An8(_) => 8,
            _ => 4,
        }
    }
}

#[derive(Debug)]
pub enum ADDInstruction {
    AR8(Register8),
    AHL,
    An8(u8),
    SPe8(i8),
    HLR16(Register16),
    HLSP,
}

impl Instruction for ADDInstruction {
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
            ADDInstruction::An8(value) => dest.write(&[0xC6, *value]),
            ADDInstruction::SPe8(value) => dest.write(&[0xE8, (*value as u8)]),
            ADDInstruction::HLR16(register16) => match register16 {
                Register16::BC => dest.write(&[0x09]),
                Register16::DE => dest.write(&[0x19]),
                Register16::HL => dest.write(&[0x29]),
            },
            ADDInstruction::HLSP => dest.write(&[0x39]),
        }
    }

    fn duration(&self) -> usize {
        match self {
            ADDInstruction::HLR16(_) | ADDInstruction::HLSP | ADDInstruction::AHL => 8,
            ADDInstruction::AR8(_) => 4,
            ADDInstruction::An8(_) => 8,
            ADDInstruction::SPe8(_) => 16,
        }
    }
}

#[derive(Debug)]
enum CPInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for CPInstruction {
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

    fn duration(&self) -> usize {
        match self {
            CPInstruction::AR8(_) => 4,
            CPInstruction::AHL => 8,
            CPInstruction::AN8(_) => 8,
        }
    }
}

#[derive(Debug)]
enum DECInstruction {
    R8(Register8),
    HL,
    R16(Register16),
    SP,
}

impl Instruction for DECInstruction {
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

    fn duration(&self) -> usize {
        match self {
            DECInstruction::R8(_) => 4,
            DECInstruction::HL => 12,
            DECInstruction::R16(_) => 8,
            DECInstruction::SP => 8,
        }
    }
}

#[derive(Debug)]
enum INCInstruction {
    R8(Register8),
    HL,
    R16(Register16),
    SP,
}

impl Instruction for INCInstruction {
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

    fn duration(&self) -> usize {
        match self {
            INCInstruction::R8(_) => 4,
            INCInstruction::HL => 12,
            INCInstruction::R16(_) => 8,
            INCInstruction::SP => 8,
        }
    }
}

#[derive(Debug)]
enum SBCInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for SBCInstruction {
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

    fn duration(&self) -> usize {
        todo!()
    }
}

#[derive(Debug)]
enum SUBInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for SUBInstruction {
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

    fn duration(&self) -> usize {
        match self {
            SUBInstruction::AR8(_) => 4,
            SUBInstruction::AHL => 8,
            SUBInstruction::AN8(_) => 8,
        }
    }
}

#[derive(Debug)]
enum ANDInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for ANDInstruction {
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

    fn duration(&self) -> usize {
        match self {
            ANDInstruction::AR8(_) => 4,
            ANDInstruction::AHL => 8,
            ANDInstruction::AN8(_) => 8,
        }
    }
}

#[derive(Debug)]
enum CPLInstruction {}

impl Instruction for CPLInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x2F])
    }

    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
enum ORInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for ORInstruction {
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

    fn duration(&self) -> usize {
        match self {
            ORInstruction::AR8(_) => 4,
            ORInstruction::AHL => 8,
            ORInstruction::AN8(_) => 8,
        }
    }
}

#[derive(Debug)]
enum XORInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for XORInstruction {
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

    fn duration(&self) -> usize {
        match self {
            XORInstruction::AR8(_) => 4,
            XORInstruction::AHL => 8,
            XORInstruction::AN8(_) => 8,
        }
    }
}

#[derive(Debug)]
enum BITInstruction {
    U3R8(BitOffset, Register8),
    U3HL(BitOffset),
}

impl Instruction for BITInstruction {
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

    fn duration(&self) -> usize {
        match self {
            BITInstruction::U3R8(_, _) => 8,
            BITInstruction::U3HL(_) => 12,
        }
    }
}

#[derive(Debug)]
enum RESInstruction {
    U3R8(BitOffset, Register8),
    U3HL(BitOffset),
}

impl Instruction for RESInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            RESInstruction::U3R8(bit_offset, register8) => match register8 {
                Register8::B => dest.write(&[0xCB, 0x80 + (*bit_offset as u8) * 8]),
                Register8::C => dest.write(&[0xCB, 0x81 + (*bit_offset as u8) * 8]),
                Register8::D => dest.write(&[0xCB, 0x82 + (*bit_offset as u8) * 8]),
                Register8::E => dest.write(&[0xCB, 0x83 + (*bit_offset as u8) * 8]),
                Register8::H => dest.write(&[0xCB, 0x84 + (*bit_offset as u8) * 8]),
                Register8::L => dest.write(&[0xCB, 0x85 + (*bit_offset as u8) * 8]),
                Register8::A => dest.write(&[0xCB, 0x87 + (*bit_offset as u8) * 8]),
            },
            RESInstruction::U3HL(bit_offset) => dest.write(&[0xCB, 0x86 + (*bit_offset as u8) * 8]),
        }
    }

    fn duration(&self) -> usize {
        match self {
            RESInstruction::U3R8(_, _) => 8,
            RESInstruction::U3HL(_) => 12,
        }
    }
}

#[derive(Debug)]
enum SETInstruction {
    U3R8(BitOffset, Register8),
    U3HL(BitOffset),
}

impl Instruction for SETInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            SETInstruction::U3R8(bit_offset, register8) => match register8 {
                Register8::B => dest.write(&[0xCB, 0xC0 + (*bit_offset as u8) * 8]),
                Register8::C => dest.write(&[0xCB, 0xC1 + (*bit_offset as u8) * 8]),
                Register8::D => dest.write(&[0xCB, 0xC2 + (*bit_offset as u8) * 8]),
                Register8::E => dest.write(&[0xCB, 0xC3 + (*bit_offset as u8) * 8]),
                Register8::H => dest.write(&[0xCB, 0xC4 + (*bit_offset as u8) * 8]),
                Register8::L => dest.write(&[0xCB, 0xC5 + (*bit_offset as u8) * 8]),
                Register8::A => dest.write(&[0xCB, 0xC7 + (*bit_offset as u8) * 8]),
            },
            SETInstruction::U3HL(bit_offset) => dest.write(&[0xCB, 0xC6 + (*bit_offset as u8) * 8]),
        }
    }

    fn duration(&self) -> usize {
        match self {
            SETInstruction::U3R8(_, _) => 8,
            SETInstruction::U3HL(_) => 12,
        }
    }
}

#[derive(Debug)]
enum RLInstruction {
    R8(Register8),
    HL,
}

impl Instruction for RLInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            RLInstruction::R8(register8) => match register8 {
                Register8::B => dest.write(&[0xCB, 0x10]),
                Register8::C => dest.write(&[0xCB, 0x11]),
                Register8::D => dest.write(&[0xCB, 0x12]),
                Register8::E => dest.write(&[0xCB, 0x13]),
                Register8::H => dest.write(&[0xCB, 0x14]),
                Register8::L => dest.write(&[0xCB, 0x15]),
                Register8::A => dest.write(&[0xCB, 0x17]),
            },
            RLInstruction::HL => dest.write(&[0xCB, 0x16]),
        }
    }

    fn duration(&self) -> usize {
        match self {
            RLInstruction::R8(_) => 8,
            RLInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
enum RLAInstruction {}

impl Instruction for RLAInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x17])
    }

    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
enum RLCInstruction {
    R8(Register8),
    HL,
}

impl Instruction for RLCInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            RLCInstruction::R8(register8) => match register8 {
                Register8::B => dest.write(&[0xCB, 0x00]),
                Register8::C => dest.write(&[0xCB, 0x01]),
                Register8::D => dest.write(&[0xCB, 0x02]),
                Register8::E => dest.write(&[0xCB, 0x03]),
                Register8::H => dest.write(&[0xCB, 0x04]),
                Register8::L => dest.write(&[0xCB, 0x05]),
                Register8::A => dest.write(&[0xCB, 0x07]),
            },
            RLCInstruction::HL => dest.write(&[0xCB, 0x06]),
        }
    }

    fn duration(&self) -> usize {
        match self {
            RLCInstruction::R8(_) => 8,
            RLCInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
enum RLCAInstruction {}

impl Instruction for RLCAInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x07])
    }

    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
enum RRInstruction {
    R8(Register8),
    HL,
}

impl Instruction for RRInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            RRInstruction::R8(register8) => match register8 {
                Register8::B => dest.write(&[0xCB, 0x18]),
                Register8::C => dest.write(&[0xCB, 0x19]),
                Register8::D => dest.write(&[0xCB, 0x1A]),
                Register8::E => dest.write(&[0xCB, 0x1B]),
                Register8::H => dest.write(&[0xCB, 0x1C]),
                Register8::L => dest.write(&[0xCB, 0x1D]),
                Register8::A => dest.write(&[0xCB, 0x1F]),
            },
            RRInstruction::HL => dest.write(&[0xCB, 0x1E]),
        }
    }

    fn duration(&self) -> usize {
        match self {
            RRInstruction::R8(_) => 8,
            RRInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
enum RRAInstruction {}

impl Instruction for RRAInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x1F])
    }

    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
enum RRCInstruction {
    R8(Register8),
    HL,
}

impl Instruction for RRCInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            RRCInstruction::R8(register8) => match register8 {
                Register8::B => dest.write(&[0xCB, 0x08]),
                Register8::C => dest.write(&[0xCB, 0x09]),
                Register8::D => dest.write(&[0xCB, 0x0A]),
                Register8::E => dest.write(&[0xCB, 0x0B]),
                Register8::H => dest.write(&[0xCB, 0x0C]),
                Register8::L => dest.write(&[0xCB, 0x0D]),
                Register8::A => dest.write(&[0xCB, 0x0F]),
            },
            RRCInstruction::HL => dest.write(&[0xCB, 0x0E]),
        }
    }

    fn duration(&self) -> usize {
        match self {
            RRCInstruction::R8(_) => 8,
            RRCInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
enum RRCAInstruction {}

impl Instruction for RRCAInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x0F])
    }

    fn duration(&self) -> usize {
        4
    }
}

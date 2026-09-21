use std::{fmt::Display, io::Write};

use crate::cpu::{
    instructions::{Assemble, BitOffset, InstructionBehavior},
    registers::Register8,
};

#[derive(Debug)]
pub enum CPLInstruction {
    Empty,
}

impl InstructionBehavior for CPLInstruction {
    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for CPLInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x2F])
    }
}

impl Display for CPLInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CPL")
    }
}

#[derive(Debug)]
pub enum RESInstruction {
    U3R8(BitOffset, Register8),
    U3HL(BitOffset),
}

impl InstructionBehavior for RESInstruction {
    fn duration(&self) -> usize {
        match self {
            RESInstruction::U3R8(_, _) => 8,
            RESInstruction::U3HL(_) => 12,
        }
    }
}

impl Assemble for RESInstruction {
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
}

impl Display for RESInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RESInstruction::U3R8(bit_offset, register8) => f.write_fmt(format_args!(
                "RES {}, {}",
                *bit_offset as u8,
                register8.as_ref()
            )),
            RESInstruction::U3HL(bit_offset) => {
                f.write_fmt(format_args!("RES {}, [HL]", *bit_offset as u8))
            }
        }
    }
}

#[derive(Debug)]
pub enum RLInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for RLInstruction {
    fn duration(&self) -> usize {
        match self {
            RLInstruction::R8(_) => 8,
            RLInstruction::HL => 16,
        }
    }
}

impl Assemble for RLInstruction {
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
}

impl Display for RLInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RLInstruction::R8(register8) => f.write_fmt(format_args!("RL {}", register8.as_ref())),
            RLInstruction::HL => f.write_str("RL [HL]"),
        }
    }
}

#[derive(Debug)]
pub enum RLAInstruction {
    Empty,
}

impl InstructionBehavior for RLAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for RLAInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x17])
    }
}

impl Display for RLAInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RLA")
    }
}

#[derive(Debug)]
pub enum RLCInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for RLCInstruction {
    fn duration(&self) -> usize {
        match self {
            RLCInstruction::R8(_) => 8,
            RLCInstruction::HL => 16,
        }
    }
}

impl Assemble for RLCInstruction {
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
}

impl Display for RLCInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RLCInstruction::R8(register8) => {
                f.write_fmt(format_args!("RLC {}", register8.as_ref()))
            }
            RLCInstruction::HL => f.write_str("RLC [HL]"),
        }
    }
}

#[derive(Debug)]
pub enum RLCAInstruction {
    Empty,
}

impl InstructionBehavior for RLCAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for RLCAInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x07])
    }
}

impl Display for RLCAInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RLCA")
    }
}

#[derive(Debug)]
pub enum RRInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for RRInstruction {
    fn duration(&self) -> usize {
        match self {
            RRInstruction::R8(_) => 8,
            RRInstruction::HL => 16,
        }
    }
}

impl Assemble for RRInstruction {
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
}

impl Display for RRInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RRInstruction::R8(register8) => f.write_fmt(format_args!("RR {}", register8.as_ref())),
            RRInstruction::HL => f.write_str("RR [HL]"),
        }
    }
}

#[derive(Debug)]
pub enum RRAInstruction {
    Empty,
}

impl InstructionBehavior for RRAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for RRAInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x1F])
    }
}

impl Display for RRAInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RRA")
    }
}

#[derive(Debug)]
pub enum RRCInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for RRCInstruction {
    fn duration(&self) -> usize {
        match self {
            RRCInstruction::R8(_) => 8,
            RRCInstruction::HL => 16,
        }
    }
}

impl Assemble for RRCInstruction {
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
}

impl Display for RRCInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RRCInstruction::R8(register8) => {
                f.write_fmt(format_args!("RRC {}", register8.as_ref()))
            }
            RRCInstruction::HL => f.write_str("RRC [HL]"),
        }
    }
}

#[derive(Debug)]
pub enum RRCAInstruction {
    Empty,
}

impl InstructionBehavior for RRCAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for RRCAInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x0F])
    }
}

impl Display for RRCAInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RRCA")
    }
}

#[derive(Debug)]
pub enum SLAInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for SLAInstruction {
    fn duration(&self) -> usize {
        match self {
            SLAInstruction::R8(_) => 8,
            SLAInstruction::HL => 16,
        }
    }
}

impl Assemble for SLAInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            SLAInstruction::R8(register8) => match register8 {
                Register8::B => dest.write(&[0xCB, 0x20]),
                Register8::C => dest.write(&[0xCB, 0x21]),
                Register8::D => dest.write(&[0xCB, 0x22]),
                Register8::E => dest.write(&[0xCB, 0x23]),
                Register8::H => dest.write(&[0xCB, 0x24]),
                Register8::L => dest.write(&[0xCB, 0x25]),
                Register8::A => dest.write(&[0xCB, 0x27]),
            },
            SLAInstruction::HL => dest.write(&[0xCB, 0x26]),
        }
    }
}

impl Display for SLAInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SLAInstruction::R8(register8) => {
                f.write_fmt(format_args!("SLA {}", register8.as_ref()))
            }
            SLAInstruction::HL => f.write_str("SLA [HL]"),
        }
    }
}

#[derive(Debug)]
pub enum SETInstruction {
    U3R8(BitOffset, Register8),
    U3HL(BitOffset),
}

impl InstructionBehavior for SETInstruction {
    fn duration(&self) -> usize {
        match self {
            SETInstruction::U3R8(_, _) => 8,
            SETInstruction::U3HL(_) => 12,
        }
    }
}

impl Assemble for SETInstruction {
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
}

impl Display for SETInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SETInstruction::U3R8(bit_offset, register8) => f.write_fmt(format_args!(
                "SET {}, {}",
                *bit_offset as u8,
                register8.as_ref()
            )),
            SETInstruction::U3HL(bit_offset) => {
                f.write_fmt(format_args!("SET {}, [HL]", *bit_offset as u8))
            }
        }
    }
}

#[derive(Debug)]
pub enum SRAInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for SRAInstruction {
    fn duration(&self) -> usize {
        match self {
            SRAInstruction::R8(_) => 8,
            SRAInstruction::HL => 16,
        }
    }
}

impl Assemble for SRAInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            SRAInstruction::R8(register8) => match register8 {
                Register8::B => dest.write(&[0xCB, 0x28]),
                Register8::C => dest.write(&[0xCB, 0x29]),
                Register8::D => dest.write(&[0xCB, 0x2A]),
                Register8::E => dest.write(&[0xCB, 0x2B]),
                Register8::H => dest.write(&[0xCB, 0x2C]),
                Register8::L => dest.write(&[0xCB, 0x2D]),
                Register8::A => dest.write(&[0xCB, 0x2F]),
            },
            SRAInstruction::HL => dest.write(&[0xCB, 0x2E]),
        }
    }
}

impl Display for SRAInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SRAInstruction::R8(register8) => {
                f.write_fmt(format_args!("SRA {}", register8.as_ref()))
            }
            SRAInstruction::HL => f.write_str("SRA [HL]"),
        }
    }
}

#[derive(Debug)]
pub enum SRLInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for SRLInstruction {
    fn duration(&self) -> usize {
        match self {
            SRLInstruction::R8(_) => 8,
            SRLInstruction::HL => 16,
        }
    }
}

impl Assemble for SRLInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            SRLInstruction::R8(register8) => match register8 {
                Register8::B => dest.write(&[0xCB, 0x38]),
                Register8::C => dest.write(&[0xCB, 0x39]),
                Register8::D => dest.write(&[0xCB, 0x3A]),
                Register8::E => dest.write(&[0xCB, 0x3B]),
                Register8::H => dest.write(&[0xCB, 0x3C]),
                Register8::L => dest.write(&[0xCB, 0x3D]),
                Register8::A => dest.write(&[0xCB, 0x3F]),
            },
            SRLInstruction::HL => dest.write(&[0xCB, 0x3E]),
        }
    }
}

impl Display for SRLInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SRLInstruction::R8(register8) => {
                f.write_fmt(format_args!("SRL {}", register8.as_ref()))
            }
            SRLInstruction::HL => f.write_str("SRL [HL]"),
        }
    }
}

#[derive(Debug)]
pub enum SWAPInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for SWAPInstruction {
    fn duration(&self) -> usize {
        match self {
            SWAPInstruction::R8(_) => 8,
            SWAPInstruction::HL => 16,
        }
    }
}

impl Assemble for SWAPInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            SWAPInstruction::R8(register8) => match register8 {
                Register8::B => dest.write(&[0xCB, 0x30]),
                Register8::C => dest.write(&[0xCB, 0x31]),
                Register8::D => dest.write(&[0xCB, 0x32]),
                Register8::E => dest.write(&[0xCB, 0x33]),
                Register8::H => dest.write(&[0xCB, 0x34]),
                Register8::L => dest.write(&[0xCB, 0x35]),
                Register8::A => dest.write(&[0xCB, 0x37]),
            },
            SWAPInstruction::HL => dest.write(&[0xCB, 0x36]),
        }
    }
}

impl Display for SWAPInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SWAPInstruction::R8(register8) => {
                f.write_fmt(format_args!("SWAP {}", register8.as_ref()))
            }
            SWAPInstruction::HL => f.write_str("SWAP [HL]"),
        }
    }
}

use std::{fmt::Display, io::Write};

use crate::cpu::{
    instructions::{Assemble, InstructionBehavior},
    registers::Register16,
};

#[derive(Debug)]
pub enum POPInstruction {
    AF,
    R16(Register16),
}

impl InstructionBehavior for POPInstruction {
    fn duration(&self) -> usize {
        12
    }
}

impl Assemble for POPInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            POPInstruction::AF => dest.write(&[0xF1]),
            POPInstruction::R16(register16) => match register16 {
                Register16::BC => dest.write(&[0xC1]),
                Register16::DE => dest.write(&[0xD1]),
                Register16::HL => dest.write(&[0xE1]),
            },
        }
    }
}

impl Display for POPInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            POPInstruction::AF => f.write_str("POP AF"),
            POPInstruction::R16(register16) => {
                f.write_fmt(format_args!("POP {}", register16.as_ref()))
            }
        }
    }
}

#[derive(Debug)]
pub enum PUSHInstruction {
    AF,
    R16(Register16),
}

impl InstructionBehavior for PUSHInstruction {
    fn duration(&self) -> usize {
        16
    }
}

impl Assemble for PUSHInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            PUSHInstruction::AF => dest.write(&[0xF5]),
            PUSHInstruction::R16(register16) => match register16 {
                Register16::BC => dest.write(&[0xC5]),
                Register16::DE => dest.write(&[0xD5]),
                Register16::HL => dest.write(&[0xE5]),
            },
        }
    }
}

impl Display for PUSHInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PUSHInstruction::AF => f.write_str("PUSH AF"),
            PUSHInstruction::R16(register16) => {
                f.write_fmt(format_args!("PUSH {}", register16.as_ref()))
            }
        }
    }
}

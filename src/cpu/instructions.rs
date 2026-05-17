use strum::FromRepr;

use crate::cpu::{
    asm::Assemble,
    operand::ConditionCode,
    registers::{Register8, Register16},
};

#[derive(Debug, FromRepr, Copy, Clone)]
#[repr(u8)]
pub enum BitOffset {
    Bit0 = 0,
    Bit1 = 1,
    Bit2 = 2,
    Bit3 = 3,
    Bit4 = 4,
    Bit5 = 5,
    Bit6 = 6,
    Bit7 = 7,
}

#[derive(Debug, FromRepr, Copy, Clone)]
#[repr(u8)]
pub enum ResetVector {
    Addr00 = 0x00,
    Addr08 = 0x08,
    Addr10 = 0x10,
    Addr18 = 0x18,
    Addr20 = 0x20,
    Addr28 = 0x28,
    Addr30 = 0x30,
    Addr38 = 0x38,
}

pub trait Instruction: Assemble {
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
pub enum CPInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for CPInstruction {
    fn duration(&self) -> usize {
        match self {
            CPInstruction::AR8(_) => 4,
            CPInstruction::AHL => 8,
            CPInstruction::AN8(_) => 8,
        }
    }
}

#[derive(Debug)]
pub enum DECInstruction {
    R8(Register8),
    HL,
    R16(Register16),
    SP,
}

impl Instruction for DECInstruction {
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
pub enum INCInstruction {
    R8(Register8),
    HL,
    R16(Register16),
    SP,
}

impl Instruction for INCInstruction {
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
pub enum SBCInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for SBCInstruction {
    fn duration(&self) -> usize {
        todo!()
    }
}

#[derive(Debug)]
pub enum SUBInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for SUBInstruction {
    fn duration(&self) -> usize {
        match self {
            SUBInstruction::AR8(_) => 4,
            SUBInstruction::AHL => 8,
            SUBInstruction::AN8(_) => 8,
        }
    }
}

#[derive(Debug)]
pub enum ANDInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for ANDInstruction {
    fn duration(&self) -> usize {
        match self {
            ANDInstruction::AR8(_) => 4,
            ANDInstruction::AHL => 8,
            ANDInstruction::AN8(_) => 8,
        }
    }
}

#[derive(Debug)]
pub enum CPLInstruction {}

impl Instruction for CPLInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum ORInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for ORInstruction {
    fn duration(&self) -> usize {
        match self {
            ORInstruction::AR8(_) => 4,
            ORInstruction::AHL => 8,
            ORInstruction::AN8(_) => 8,
        }
    }
}

#[derive(Debug)]
pub enum XORInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl Instruction for XORInstruction {
    fn duration(&self) -> usize {
        match self {
            XORInstruction::AR8(_) => 4,
            XORInstruction::AHL => 8,
            XORInstruction::AN8(_) => 8,
        }
    }
}

#[derive(Debug)]
pub enum BITInstruction {
    U3R8(BitOffset, Register8),
    U3HL(BitOffset),
}

impl Instruction for BITInstruction {
    fn duration(&self) -> usize {
        match self {
            BITInstruction::U3R8(_, _) => 8,
            BITInstruction::U3HL(_) => 12,
        }
    }
}

#[derive(Debug)]
pub enum RESInstruction {
    U3R8(BitOffset, Register8),
    U3HL(BitOffset),
}

impl Instruction for RESInstruction {
    fn duration(&self) -> usize {
        match self {
            RESInstruction::U3R8(_, _) => 8,
            RESInstruction::U3HL(_) => 12,
        }
    }
}

#[derive(Debug)]
pub enum SETInstruction {
    U3R8(BitOffset, Register8),
    U3HL(BitOffset),
}

impl Instruction for SETInstruction {
    fn duration(&self) -> usize {
        match self {
            SETInstruction::U3R8(_, _) => 8,
            SETInstruction::U3HL(_) => 12,
        }
    }
}

#[derive(Debug)]
pub enum RLInstruction {
    R8(Register8),
    HL,
}

impl Instruction for RLInstruction {
    fn duration(&self) -> usize {
        match self {
            RLInstruction::R8(_) => 8,
            RLInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum RLAInstruction {}

impl Instruction for RLAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum RLCInstruction {
    R8(Register8),
    HL,
}

impl Instruction for RLCInstruction {
    fn duration(&self) -> usize {
        match self {
            RLCInstruction::R8(_) => 8,
            RLCInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum RLCAInstruction {}

impl Instruction for RLCAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum RRInstruction {
    R8(Register8),
    HL,
}

impl Instruction for RRInstruction {
    fn duration(&self) -> usize {
        match self {
            RRInstruction::R8(_) => 8,
            RRInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum RRAInstruction {}

impl Instruction for RRAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum RRCInstruction {
    R8(Register8),
    HL,
}

impl Instruction for RRCInstruction {
    fn duration(&self) -> usize {
        match self {
            RRCInstruction::R8(_) => 8,
            RRCInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum RRCAInstruction {}

impl Instruction for RRCAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum SLAInstruction {
    R8(Register8),
    HL,
}

impl Instruction for SLAInstruction {
    fn duration(&self) -> usize {
        match self {
            SLAInstruction::R8(_) => 8,
            SLAInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum SRAInstruction {
    R8(Register8),
    HL,
}

impl Instruction for SRAInstruction {
    fn duration(&self) -> usize {
        match self {
            SRAInstruction::R8(_) => 8,
            SRAInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum SRLInstruction {
    R8(Register8),
    HL,
}

impl Instruction for SRLInstruction {
    fn duration(&self) -> usize {
        match self {
            SRLInstruction::R8(_) => 8,
            SRLInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum SWAPInstruction {
    R8(Register8),
    HL,
}

impl Instruction for SWAPInstruction {
    fn duration(&self) -> usize {
        match self {
            SWAPInstruction::R8(_) => 8,
            SWAPInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum CALLInstruction {
    N16(u16),
    CCN16(ConditionCode, u16),
}

impl Instruction for CALLInstruction {
    fn duration(&self) -> usize {
        match self {
            CALLInstruction::N16(_) => 24,
            CALLInstruction::CCN16(_, _) => 12,
        }
    }
}

#[derive(Debug)]
pub enum JPInstruction {
    HL,
    N16(u16),
    CCN16(ConditionCode, u16),
}

impl Instruction for JPInstruction {
    fn duration(&self) -> usize {
        match self {
            JPInstruction::HL => 4,
            JPInstruction::N16(_) => 16,
            JPInstruction::CCN16(_, _) => 12,
        }
    }
}

#[derive(Debug)]
pub enum JRInstruction {
    E8(i8),
    CCE8(ConditionCode, i8),
}

impl Instruction for JRInstruction {
    fn duration(&self) -> usize {
        match self {
            JRInstruction::E8(_) => 12,
            JRInstruction::CCE8(_, _) => 8,
        }
    }
}

#[derive(Debug)]
pub enum RETInstruction {
    Conditional(ConditionCode),
    Unconditional,
}

impl Instruction for RETInstruction {
    fn duration(&self) -> usize {
        match self {
            RETInstruction::Conditional(_) => 8,
            RETInstruction::Unconditional => 16,
        }
    }
}

#[derive(Debug)]
pub enum RETIInstruction {}

impl Instruction for RETIInstruction {
    fn duration(&self) -> usize {
        16
    }
}

pub enum RSTInstruction {
    ResetVector(ResetVector),
}

impl Instruction for RSTInstruction {
    fn duration(&self) -> usize {
        16
    }
}

#[derive(Debug)]
pub enum CCFInstruction {}

impl Instruction for CCFInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum SCFInstruction {}

impl Instruction for SCFInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum POPInstruction {
    AF,
    R16(Register16),
}

impl Instruction for POPInstruction {
    fn duration(&self) -> usize {
        12
    }
}

#[derive(Debug)]
pub enum PUSHInstruction {
    AF,
    R16(Register16),
}

impl Instruction for PUSHInstruction {
    fn duration(&self) -> usize {
        16
    }
}

#[derive(Debug)]
pub enum DIInstruction {}

impl Instruction for DIInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum EIInstruction {}

impl Instruction for EIInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum HALTInstruction {}

impl Instruction for HALTInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum DAAInstruction {}

impl Instruction for DAAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum NOPInstruction {}

impl Instruction for NOPInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum STOPInstruction {}

impl Instruction for STOPInstruction {
    fn duration(&self) -> usize {
        4
    }
}

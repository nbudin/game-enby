use enum_dispatch::enum_dispatch;
use std::{
    any::type_name,
    fmt::{Debug, Display},
    io::Write,
    sync::{Arc, RwLock},
};
use strum::FromRepr;

use crate::cpu::{
    CPU,
    asm::Assemble,
    cpu_bus::CPUBusTrait,
    instructions::{
        arithmetic::{
            ADCInstruction, ADDInstruction, DAAInstruction, DECInstruction, INCInstruction,
            SBCInstruction, SUBInstruction,
        },
        bitwise::{
            CPLInstruction, RESInstruction, RLAInstruction, RLCAInstruction, RLCInstruction,
            RLInstruction, RRAInstruction, RRCAInstruction, RRCInstruction, RRInstruction,
            SETInstruction, SLAInstruction, SRAInstruction, SRLInstruction, SWAPInstruction,
        },
        control::{
            CALLInstruction, HALTInstruction, JPInstruction, JRInstruction, NOPInstruction,
            RETIInstruction, RETInstruction, RSTInstruction, STOPInstruction,
        },
        flags::{CCFInstruction, DIInstruction, EIInstruction, SCFInstruction},
        load::{LDHInstruction, LDInstruction},
        logic::{ANDInstruction, ORInstruction, XORInstruction},
        stack::{POPInstruction, PUSHInstruction},
        test::{BITInstruction, CPInstruction},
    },
};

pub mod arithmetic;
pub mod bitwise;
pub mod control;
pub mod flags;
pub mod load;
pub mod logic;
pub mod stack;
pub mod test;

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

#[enum_dispatch]
pub trait InstructionBehavior: Assemble + Display {
    fn duration(&self) -> usize;

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        todo!("{}", type_name::<Self>());
    }
}

#[enum_dispatch(Assemble, InstructionBehavior)]
#[derive(Debug, derive_more::Display)]
pub enum Instruction {
    ADCInstruction,
    ADDInstruction,
    ANDInstruction,
    BITInstruction,
    CALLInstruction,
    CCFInstruction,
    CPInstruction,
    CPLInstruction,
    DAAInstruction,
    DECInstruction,
    DIInstruction,
    EIInstruction,
    HALTInstruction,
    INCInstruction,
    JPInstruction,
    JRInstruction,
    LDHInstruction,
    LDInstruction,
    NOPInstruction,
    ORInstruction,
    POPInstruction,
    PUSHInstruction,
    RESInstruction,
    RETIInstruction,
    RETInstruction,
    RLAInstruction,
    RLCAInstruction,
    RLCInstruction,
    RLInstruction,
    RRAInstruction,
    RRCAInstruction,
    RRCInstruction,
    RRInstruction,
    RSTInstruction,
    SBCInstruction,
    SCFInstruction,
    SETInstruction,
    SLAInstruction,
    SRAInstruction,
    SRLInstruction,
    STOPInstruction,
    SUBInstruction,
    SWAPInstruction,
    XORInstruction,
}

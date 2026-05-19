use std::io::{Cursor, Read, Seek, SeekFrom};

use crate::cpu::registers::{AFRegister, BCRegister, CPURegisters, DERegister, HLRegister};

pub mod asm;
pub mod cpu_bus;
pub mod disasm;
pub mod instructions;
pub mod operand;
pub mod registers;

pub struct CPU {
    pub registers: CPURegisters,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: CPURegisters {
                af: AFRegister::from_bits(0),
                bc: BCRegister::from_bits(0),
                de: DERegister::from_bits(0),
                hl: HLRegister::from_bits(0),
                sp: 0,
                pc: 0x0100,
            },
        }
    }
}

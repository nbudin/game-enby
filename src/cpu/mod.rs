use crate::cpu::registers::CPURegisters;

pub mod instructions;
pub mod operand;
pub mod registers;

pub struct CPU {
    pub registers: CPURegisters,
}

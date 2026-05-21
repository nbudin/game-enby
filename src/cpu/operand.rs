use crate::cpu::{
    CPU,
    registers::{CPURegisters, Register8, Register16},
};

#[derive(Debug)]
pub enum ConditionCode {
    Z,
    NZ,
    C,
    NC,
}

impl ConditionCode {
    pub fn matches(&self, registers: &CPURegisters) -> bool {
        match self {
            ConditionCode::Z => registers.af.f().z(),
            ConditionCode::NZ => !registers.af.f().z(),
            ConditionCode::C => registers.af.f().c(),
            ConditionCode::NC => !registers.af.f().c(),
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Register8(Register8),
    Register16(Register16),
    Int8(u8),
    Int16(u16),
    Offset(i8),
    BitIndex(u8),
    ConditionCode(ConditionCode),
    RSTVector(u8),
}

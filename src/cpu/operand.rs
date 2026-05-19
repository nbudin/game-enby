use crate::cpu::{
    CPU,
    registers::{Register8, Register16},
};

#[derive(Debug)]
pub enum ConditionCode {
    Z,
    NZ,
    C,
    NC,
}

impl ConditionCode {
    pub fn matches(&self, cpu: &CPU) -> bool {
        match self {
            ConditionCode::Z => cpu.registers.af.f().z(),
            ConditionCode::NZ => !cpu.registers.af.f().z(),
            ConditionCode::C => cpu.registers.af.f().c(),
            ConditionCode::NC => !cpu.registers.af.f().c(),
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

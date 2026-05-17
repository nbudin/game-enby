use crate::cpu::registers::{Register8, Register16};

#[derive(Debug)]
pub enum ConditionCode {
    Z,
    NZ,
    C,
    NC,
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

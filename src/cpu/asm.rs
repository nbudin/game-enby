use std::io::Write;

use bytemuck::bytes_of;
use enum_dispatch::enum_dispatch;
use zendian::le::u16le;

use crate::cpu::{
    instructions::*,
    operand::ConditionCode,
    registers::{Register8, Register16},
};

#[enum_dispatch]
pub trait Assemble {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error>;
}

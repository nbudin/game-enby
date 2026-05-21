pub mod asm;
pub mod cpu_bus;
pub mod disasm;
pub mod instructions;
pub mod operand;
pub mod registers;

pub struct CPU {}

impl CPU {
    pub fn new() -> CPU {
        CPU {}
    }
}

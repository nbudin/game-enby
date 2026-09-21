use std::{
    fmt::Display,
    io::Write,
    sync::{Arc, RwLock},
};

use bytemuck::bytes_of;
use zendian::le::u16le;

use crate::cpu::{
    CPU,
    cpu_bus::CPUBusTrait,
    instructions::{Assemble, EIInstruction, InstructionBehavior, ResetVector},
    operand::ConditionCode,
};

#[derive(Debug)]
pub enum CALLInstruction {
    N16(u16),
    CCN16(ConditionCode, u16),
}

impl CALLInstruction {
    fn perform_call(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait, addr: u16) {
        let pc = cpu.read().unwrap().registers.pc;
        let new_sp = cpu.read().unwrap().registers.sp - 2;
        cpu_bus.write(new_sp - 1, (pc & 0xFF) as u8);
        cpu_bus.write(new_sp, (pc >> 8) as u8);

        let registers = &mut cpu.write().unwrap().registers;
        registers.sp = new_sp;
        registers.pc = addr;
    }
}

impl InstructionBehavior for CALLInstruction {
    fn duration(&self) -> usize {
        match self {
            CALLInstruction::N16(_) => 24,
            CALLInstruction::CCN16(_, _) => 12,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            CALLInstruction::N16(addr) => self.perform_call(cpu, cpu_bus, *addr),
            CALLInstruction::CCN16(condition_code, addr) => {
                if condition_code.matches(&cpu.read().unwrap().registers) {
                    self.perform_call(cpu, cpu_bus, *addr);
                }
            }
        }
    }
}

impl Assemble for CALLInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            CALLInstruction::N16(addr) => {
                Ok(dest.write(&[0xCD])? + dest.write(bytes_of(&u16le::from(addr)))?)
            }
            CALLInstruction::CCN16(condition_code, addr) => match condition_code {
                ConditionCode::NZ => {
                    Ok(dest.write(&[0xC4])? + dest.write(bytes_of(&u16le::from(addr)))?)
                }
                ConditionCode::Z => {
                    Ok(dest.write(&[0xCC])? + dest.write(bytes_of(&u16le::from(addr)))?)
                }
                ConditionCode::NC => {
                    Ok(dest.write(&[0xD4])? + dest.write(bytes_of(&u16le::from(addr)))?)
                }
                ConditionCode::C => {
                    Ok(dest.write(&[0xDC])? + dest.write(bytes_of(&u16le::from(addr)))?)
                }
            },
        }
    }
}

impl Display for CALLInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CALLInstruction::N16(addr) => f.write_fmt(format_args!("CALL ${:04X}", addr)),
            CALLInstruction::CCN16(condition_code, addr) => f.write_fmt(format_args!(
                "CALL {}, ${:04X}",
                condition_code.as_ref(),
                addr
            )),
        }
    }
}

#[derive(Debug)]
pub enum HALTInstruction {
    Empty,
}

impl InstructionBehavior for HALTInstruction {
    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for HALTInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x76])
    }
}

impl Display for HALTInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("HALT")
    }
}

#[derive(Debug)]
pub enum JPInstruction {
    HL,
    N16(u16),
    CCN16(ConditionCode, u16),
}

impl InstructionBehavior for JPInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, _cpu_bus: &mut dyn CPUBusTrait) {
        let target_addr = match self {
            JPInstruction::HL => Some(cpu.read().unwrap().registers.hl.into_bits()),
            JPInstruction::N16(addr) => Some(*addr),
            JPInstruction::CCN16(condition_code, addr) => {
                if condition_code.matches(&cpu.read().unwrap().registers) {
                    Some(*addr)
                } else {
                    None
                }
            }
        };

        if let Some(target_addr) = target_addr {
            cpu.write().unwrap().registers.pc = target_addr;
        }
    }

    fn duration(&self) -> usize {
        match self {
            JPInstruction::HL => 4,
            JPInstruction::N16(_) => 16,
            JPInstruction::CCN16(_, _) => 12,
        }
    }
}

impl Assemble for JPInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            JPInstruction::N16(addr) => {
                Ok(dest.write(&[0xC3])? + dest.write(bytes_of(&u16le::from(addr)))?)
            }
            JPInstruction::CCN16(condition_code, addr) => match condition_code {
                ConditionCode::NZ => {
                    Ok(dest.write(&[0xC2])? + dest.write(bytes_of(&u16le::from(addr)))?)
                }
                ConditionCode::Z => {
                    Ok(dest.write(&[0xCA])? + dest.write(bytes_of(&u16le::from(addr)))?)
                }
                ConditionCode::NC => {
                    Ok(dest.write(&[0xD2])? + dest.write(bytes_of(&u16le::from(addr)))?)
                }
                ConditionCode::C => {
                    Ok(dest.write(&[0xDA])? + dest.write(bytes_of(&u16le::from(addr)))?)
                }
            },
            JPInstruction::HL => dest.write(&[0xE9]),
        }
    }
}

impl Display for JPInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JPInstruction::HL => f.write_str("JP HL"),
            JPInstruction::N16(addr) => f.write_fmt(format_args!("JP ${:04X}", addr)),
            JPInstruction::CCN16(condition_code, addr) => {
                f.write_fmt(format_args!("JP {},${:04X}", condition_code.as_ref(), addr))
            }
        }
    }
}

#[derive(Debug)]
pub enum JRInstruction {
    E8(i8),
    CCE8(ConditionCode, i8),
}

impl JRInstruction {
    fn jump_relative(&self, cpu: Arc<RwLock<CPU>>, offset: i8) {
        let registers = &mut cpu.write().unwrap().registers;
        let new_address = registers.pc.saturating_add_signed(offset as i16);
        registers.pc = new_address;
    }
}

impl InstructionBehavior for JRInstruction {
    fn duration(&self) -> usize {
        match self {
            JRInstruction::E8(_) => 12,
            JRInstruction::CCE8(_, _) => 8,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, _cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            JRInstruction::E8(offset) => self.jump_relative(cpu, *offset),
            JRInstruction::CCE8(condition_code, offset) => {
                if condition_code.matches(&cpu.read().unwrap().registers) {
                    self.jump_relative(cpu, *offset);
                }
            }
        }
    }
}

impl Assemble for JRInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            JRInstruction::E8(offset) => dest.write(&[0x18, *offset as u8]),
            JRInstruction::CCE8(condition_code, offset) => match condition_code {
                ConditionCode::NZ => dest.write(&[0x20, *offset as u8]),
                ConditionCode::Z => dest.write(&[0x28, *offset as u8]),
                ConditionCode::NC => dest.write(&[0x30, *offset as u8]),
                ConditionCode::C => dest.write(&[0x38, *offset as u8]),
            },
        }
    }
}

impl Display for JRInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JRInstruction::E8(offset) => f.write_fmt(format_args!("JR {}", offset)),
            JRInstruction::CCE8(condition_code, offset) => {
                f.write_fmt(format_args!("JR {}, {}", condition_code.as_ref(), offset))
            }
        }
    }
}

#[derive(Debug)]
pub enum NOPInstruction {
    Empty,
}

impl InstructionBehavior for NOPInstruction {
    fn execute(&self, _cpu: Arc<RwLock<CPU>>, _cpu_bus: &mut dyn CPUBusTrait) {}

    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for NOPInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x00])
    }
}

impl Display for NOPInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NOP")
    }
}

#[derive(Debug)]
pub enum RETInstruction {
    Conditional(ConditionCode),
    Unconditional,
}

impl RETInstruction {
    fn perform_return(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        let sp = cpu.read().unwrap().registers.sp;
        let new_pc = cpu_bus.read(sp) as u16 + ((cpu_bus.read(sp + 1) as u16) << 8);

        let registers = &mut cpu.write().unwrap().registers;
        registers.sp += 2;
        registers.pc = new_pc;
    }
}

impl InstructionBehavior for RETInstruction {
    fn duration(&self) -> usize {
        match self {
            RETInstruction::Conditional(_) => 8,
            RETInstruction::Unconditional => 16,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            RETInstruction::Conditional(condition_code) => {
                if condition_code.matches(&cpu.read().unwrap().registers) {
                    self.perform_return(cpu, cpu_bus);
                }
            }
            RETInstruction::Unconditional => self.perform_return(cpu, cpu_bus),
        }
    }
}

impl Assemble for RETInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            RETInstruction::Conditional(condition_code) => match condition_code {
                ConditionCode::NZ => dest.write(&[0xC0]),
                ConditionCode::Z => dest.write(&[0xC8]),
                ConditionCode::NC => dest.write(&[0xD0]),
                ConditionCode::C => dest.write(&[0xD8]),
            },
            RETInstruction::Unconditional => dest.write(&[0xC9]),
        }
    }
}

impl Display for RETInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RETInstruction::Conditional(condition_code) => {
                f.write_fmt(format_args!("RET {}", condition_code.as_ref()))
            }
            RETInstruction::Unconditional => f.write_str("RET"),
        }
    }
}

#[derive(Debug)]
pub enum RETIInstruction {
    Empty,
}

impl InstructionBehavior for RETIInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        EIInstruction::Empty.execute(cpu.clone(), cpu_bus);
        RETInstruction::Unconditional.execute(cpu, cpu_bus);
    }

    fn duration(&self) -> usize {
        16
    }
}

impl Assemble for RETIInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0xD9])
    }
}

impl Display for RETIInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RETI")
    }
}

#[derive(Debug)]
pub enum RSTInstruction {
    ResetVector(ResetVector),
}

impl InstructionBehavior for RSTInstruction {
    fn duration(&self) -> usize {
        16
    }
}

impl Display for RSTInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RSTInstruction::ResetVector(reset_vector) => {
                f.write_fmt(format_args!("RST ${:02X}", *reset_vector as u8))
            }
        }
    }
}

impl Assemble for RSTInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        match self {
            RSTInstruction::ResetVector(reset_vector) => {
                dest.write(&[0xC0 + (*reset_vector as u8)])
            }
        }
    }
}

#[derive(Debug)]
pub enum STOPInstruction {
    Empty,
}

impl InstructionBehavior for STOPInstruction {
    fn duration(&self) -> usize {
        4
    }
}

impl Assemble for STOPInstruction {
    fn assemble(&self, dest: &mut impl Write) -> Result<usize, std::io::Error> {
        dest.write(&[0x10, 0x00])
    }
}

impl Display for STOPInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("STOP")
    }
}

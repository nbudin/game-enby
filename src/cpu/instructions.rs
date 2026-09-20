use enum_dispatch::enum_dispatch;
use std::{
    any::type_name,
    fmt::Debug,
    io::Write,
    sync::{Arc, RwLock},
};
use strum::FromRepr;

use crate::cpu::{
    CPU,
    asm::Assemble,
    cpu_bus::CPUBusTrait,
    operand::ConditionCode,
    registers::{CPUFlags, CPURegisters, HLRegister, Register8, Register16},
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

#[enum_dispatch]
pub trait InstructionBehavior: Assemble {
    fn duration(&self) -> usize;

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        todo!("{}", type_name::<Self>());
    }
}

#[enum_dispatch(Assemble, InstructionBehavior)]
#[derive(Debug)]
pub enum Instruction {
    LDInstruction,
    LDHInstruction,
    ADCInstruction,
    ADDInstruction,
    CPInstruction,
    DECInstruction,
    INCInstruction,
    SBCInstruction,
    SUBInstruction,
    ANDInstruction,
    CPLInstruction,
    ORInstruction,
    XORInstruction,
    BITInstruction,
    RESInstruction,
    SETInstruction,
    RLInstruction,
    RLAInstruction,
    RLCInstruction,
    RLCAInstruction,
    RRInstruction,
    RRAInstruction,
    RRCInstruction,
    RRCAInstruction,
    SLAInstruction,
    SRAInstruction,
    SRLInstruction,
    SWAPInstruction,
    CALLInstruction,
    JPInstruction,
    JRInstruction,
    RETInstruction,
    RETIInstruction,
    RSTInstruction,
    CCFInstruction,
    SCFInstruction,
    POPInstruction,
    PUSHInstruction,
    DIInstruction,
    EIInstruction,
    HALTInstruction,
    DAAInstruction,
    NOPInstruction,
    STOPInstruction,
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

impl InstructionBehavior for LDInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            LDInstruction::R8R8(to, from) => {
                let registers = &mut cpu.write().unwrap().registers;
                let value = registers.get_r8(*from);
                registers.set_r8(*to, value);
            }
            LDInstruction::R8N8(to, value) => cpu.write().unwrap().registers.set_r8(*to, *value),
            LDInstruction::R16N16(to, value) => cpu.write().unwrap().registers.set_r16(*to, *value),
            LDInstruction::SPN16(value) => cpu.write().unwrap().registers.sp = *value,
            LDInstruction::N16SP(_) => todo!(),
            LDInstruction::HLR8(from) => {
                let to = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu.read().unwrap().registers.get_r8(*from);
                cpu_bus.write(to, value);
            }
            LDInstruction::HLN8(value) => {
                let to = cpu.read().unwrap().registers.hl.into_bits();
                cpu_bus.write(to, *value);
            }
            LDInstruction::R8HL(to) => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                cpu.write().unwrap().registers.set_r8(*to, value);
            }
            LDInstruction::R16A(register16) => todo!(),
            LDInstruction::N16A(addr) => {
                let value = cpu.read().unwrap().registers.af.a();
                cpu_bus.write(*addr, value);
            }
            LDInstruction::AR16(register16) => todo!(),
            LDInstruction::AN16(addr) => {
                let value = cpu_bus.read(*addr);
                cpu.write().unwrap().registers.af.set_a(value);
            }
            LDInstruction::HLIA => {
                let to = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu.read().unwrap().registers.af.a();
                cpu_bus.write(to, value);
                cpu.write().unwrap().registers.hl = HLRegister::from_bits(to.wrapping_add(1));
            }
            LDInstruction::HLDA => {
                let to = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu.read().unwrap().registers.af.a();
                cpu_bus.write(to, value);
                cpu.write().unwrap().registers.hl = HLRegister::from_bits(to.wrapping_sub(1));
            }
            LDInstruction::AHLI => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                let registers = &mut cpu.write().unwrap().registers;
                registers.af.set_a(value);
                registers.hl = HLRegister::from_bits(addr.wrapping_add(1));
            }
            LDInstruction::AHLD => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                let registers = &mut cpu.write().unwrap().registers;
                registers.af.set_a(value);
                registers.hl = HLRegister::from_bits(addr.wrapping_sub(1));
            }
            LDInstruction::HLSPE8(_) => todo!(),
            LDInstruction::SPHL => todo!(),
        }
    }

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

impl InstructionBehavior for LDHInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            LDHInstruction::N8A(offset) => {
                let value = cpu.read().unwrap().registers.af.a();
                cpu_bus.write(0xFF00 + (*offset as u16), value);
            }
            LDHInstruction::CA => {
                let offset = cpu.read().unwrap().registers.bc.c();
                let value = cpu.read().unwrap().registers.af.a();
                cpu_bus.write(0xFF00 + (offset as u16), value);
            }
            LDHInstruction::AN8(offset) => {
                let value = cpu_bus.read(0xFF00 + (*offset as u16));
                cpu.write().unwrap().registers.af.set_a(value);
            }
            LDHInstruction::AC => {
                let offset = cpu.read().unwrap().registers.bc.c();
                let value = cpu_bus.read(0xFF00 + (offset as u16));
                cpu.write().unwrap().registers.af.set_a(value);
            }
        }
    }

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
    AR8(Register8),
    AHL,
    An8(u8),
}

impl InstructionBehavior for ADCInstruction {
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

impl InstructionBehavior for ADDInstruction {
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

impl CPInstruction {
    fn compare_with_a(&self, cpu: Arc<RwLock<CPU>>, value: u8) {
        let mut cpu = cpu.write().unwrap();
        let a_value = cpu.registers.af.a();
        let new_f = cpu
            .registers
            .af
            .f()
            .with_z(a_value == value)
            .with_n(true)
            .with_h(value & 0x0F > a_value & 0x0F)
            .with_c(value > a_value);
        cpu.registers.af.set_f(new_f);
    }
}

impl InstructionBehavior for CPInstruction {
    fn duration(&self) -> usize {
        match self {
            CPInstruction::AR8(_) => 4,
            CPInstruction::AHL => 8,
            CPInstruction::AN8(_) => 8,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            CPInstruction::AR8(register8) => {
                let value = cpu.read().unwrap().registers.get_r8(*register8);
                self.compare_with_a(cpu, value);
            }
            CPInstruction::AHL => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                self.compare_with_a(cpu, value)
            }
            CPInstruction::AN8(value) => {
                self.compare_with_a(cpu, *value);
            }
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

impl DECInstruction {
    fn set_flags_after_dec(&self, registers: &mut CPURegisters, new_value: u8) {
        registers.af.set_f(
            registers
                .af
                .f()
                .with_z(new_value == 0)
                .with_n(true)
                .with_h(new_value & 0x0F == 0x0F),
        );
    }
}

impl InstructionBehavior for DECInstruction {
    fn duration(&self) -> usize {
        match self {
            DECInstruction::R8(_) => 4,
            DECInstruction::HL => 12,
            DECInstruction::R16(_) => 8,
            DECInstruction::SP => 8,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            DECInstruction::R8(register8) => {
                let registers = &mut cpu.write().unwrap().registers;
                let value = registers.get_r8(*register8);
                let new_value = value.wrapping_sub(1);
                registers.set_r8(*register8, new_value);
                self.set_flags_after_dec(registers, new_value);
            }
            DECInstruction::HL => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                let new_value = value.wrapping_sub(1);
                cpu_bus.write(addr, new_value);
                self.set_flags_after_dec(&mut cpu.write().unwrap().registers, new_value);
            }
            DECInstruction::R16(register16) => {
                let registers = &mut cpu.write().unwrap().registers;
                let value = registers.get_r16(*register16);
                registers.set_r16(*register16, value.wrapping_sub(1));
            }
            DECInstruction::SP => {
                let registers = &mut cpu.write().unwrap().registers;
                registers.sp = registers.sp.wrapping_sub(1);
            }
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

impl INCInstruction {
    fn set_flags_after_inc(&self, registers: &mut CPURegisters, new_value: u8) {
        registers.af.set_f(
            registers
                .af
                .f()
                .with_z(new_value == 0)
                .with_n(false)
                .with_h(new_value & 0x0F == 0),
        );
    }
}

impl InstructionBehavior for INCInstruction {
    fn duration(&self) -> usize {
        match self {
            INCInstruction::R8(_) => 4,
            INCInstruction::HL => 12,
            INCInstruction::R16(_) => 8,
            INCInstruction::SP => 8,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            INCInstruction::R8(register8) => {
                let registers = &mut cpu.write().unwrap().registers;
                let value = registers.get_r8(*register8);
                let new_value = value.wrapping_add(1);
                registers.set_r8(*register8, new_value);
                self.set_flags_after_inc(registers, new_value);
            }
            INCInstruction::HL => {
                let addr = cpu.read().unwrap().registers.hl.into_bits();
                let value = cpu_bus.read(addr);
                let new_value = value.wrapping_add(1);
                cpu_bus.write(addr, new_value);
                self.set_flags_after_inc(&mut cpu.write().unwrap().registers, new_value);
            }
            INCInstruction::R16(register16) => {
                let registers = &mut cpu.write().unwrap().registers;
                let value = registers.get_r16(*register16);
                registers.set_r16(*register16, value.wrapping_add(1));
            }
            INCInstruction::SP => {
                let registers = &mut cpu.write().unwrap().registers;
                registers.sp = registers.sp.wrapping_add(1);
            }
        }
    }
}

#[derive(Debug)]
pub enum SBCInstruction {
    AR8(Register8),
    AHL,
    AN8(u8),
}

impl InstructionBehavior for SBCInstruction {
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

impl InstructionBehavior for SUBInstruction {
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

impl InstructionBehavior for ANDInstruction {
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

impl InstructionBehavior for CPLInstruction {
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

impl InstructionBehavior for ORInstruction {
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

impl InstructionBehavior for XORInstruction {
    fn duration(&self) -> usize {
        match self {
            XORInstruction::AR8(_) => 4,
            XORInstruction::AHL => 8,
            XORInstruction::AN8(_) => 8,
        }
    }

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        match self {
            XORInstruction::AR8(register8) => {
                let registers = &mut cpu.write().unwrap().registers;
                registers
                    .af
                    .set_a(registers.af.a() ^ registers.get_r8(*register8));
            }
            XORInstruction::AHL => {
                let bus = cpu_bus;
                let value = bus.read_readonly(cpu.read().unwrap().registers.hl.into_bits());
                let registers = &mut cpu.write().unwrap().registers;
                registers.af.set_a(registers.af.a() ^ value)
            }
            XORInstruction::AN8(value) => {
                let registers = &mut cpu.write().unwrap().registers;
                registers.af.set_a(registers.af.a() ^ value);
            }
        }
    }
}

#[derive(Debug)]
pub enum BITInstruction {
    U3R8(BitOffset, Register8),
    U3HL(BitOffset),
}

impl InstructionBehavior for BITInstruction {
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

impl InstructionBehavior for RESInstruction {
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

impl InstructionBehavior for SETInstruction {
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

impl InstructionBehavior for RLInstruction {
    fn duration(&self) -> usize {
        match self {
            RLInstruction::R8(_) => 8,
            RLInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum RLAInstruction {
    Empty,
}

impl InstructionBehavior for RLAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum RLCInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for RLCInstruction {
    fn duration(&self) -> usize {
        match self {
            RLCInstruction::R8(_) => 8,
            RLCInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum RLCAInstruction {
    Empty,
}

impl InstructionBehavior for RLCAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum RRInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for RRInstruction {
    fn duration(&self) -> usize {
        match self {
            RRInstruction::R8(_) => 8,
            RRInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum RRAInstruction {}

impl InstructionBehavior for RRAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum RRCInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for RRCInstruction {
    fn duration(&self) -> usize {
        match self {
            RRCInstruction::R8(_) => 8,
            RRCInstruction::HL => 16,
        }
    }
}

#[derive(Debug)]
pub enum RRCAInstruction {
    Empty,
}

impl InstructionBehavior for RRCAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum SLAInstruction {
    R8(Register8),
    HL,
}

impl InstructionBehavior for SLAInstruction {
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

impl InstructionBehavior for SRAInstruction {
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

impl InstructionBehavior for SRLInstruction {
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

impl InstructionBehavior for SWAPInstruction {
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

impl InstructionBehavior for CALLInstruction {
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

impl InstructionBehavior for JPInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
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

    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
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

#[derive(Debug)]
pub enum RETInstruction {
    Conditional(ConditionCode),
    Unconditional,
}

impl InstructionBehavior for RETInstruction {
    fn duration(&self) -> usize {
        match self {
            RETInstruction::Conditional(_) => 8,
            RETInstruction::Unconditional => 16,
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

#[derive(Debug)]
pub enum RSTInstruction {
    ResetVector(ResetVector),
}

impl InstructionBehavior for RSTInstruction {
    fn duration(&self) -> usize {
        16
    }
}

#[derive(Debug)]
pub enum CCFInstruction {
    Empty,
}

impl InstructionBehavior for CCFInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, _cpu_bus: &mut dyn CPUBusTrait) {
        let registers = &mut cpu.write().unwrap().registers;
        registers.af.set_f(
            registers
                .af
                .f()
                .with_c(!registers.af.f().c())
                .with_n(false)
                .with_h(false),
        );
    }

    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum SCFInstruction {}

impl InstructionBehavior for SCFInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum POPInstruction {
    AF,
    R16(Register16),
}

impl InstructionBehavior for POPInstruction {
    fn duration(&self) -> usize {
        12
    }
}

#[derive(Debug)]
pub enum PUSHInstruction {
    AF,
    R16(Register16),
}

impl InstructionBehavior for PUSHInstruction {
    fn duration(&self) -> usize {
        16
    }
}

#[derive(Debug)]
pub enum DIInstruction {
    Empty,
}

impl InstructionBehavior for DIInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        cpu.write().unwrap().interrupt_master_enable = false;
    }

    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum EIInstruction {
    Empty,
}

impl InstructionBehavior for EIInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {
        cpu.write().unwrap().interrupt_master_enable = true;
    }

    fn duration(&self) -> usize {
        4
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

#[derive(Debug)]
pub enum DAAInstruction {
    Empty,
}

impl InstructionBehavior for DAAInstruction {
    fn duration(&self) -> usize {
        4
    }
}

#[derive(Debug)]
pub enum NOPInstruction {
    Empty,
}

impl InstructionBehavior for NOPInstruction {
    fn execute(&self, cpu: Arc<RwLock<CPU>>, cpu_bus: &mut dyn CPUBusTrait) {}

    fn duration(&self) -> usize {
        4
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

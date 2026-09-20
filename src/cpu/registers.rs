use bitfield_struct::bitfield;
use strum::AsRefStr;

#[bitfield(u8)]
pub struct CPUFlags {
    #[bits(4)]
    _unused: u8,
    /// carry flag
    pub c: bool,
    /// half-carry flag (BCD)
    pub h: bool,
    /// subtraction flag (BCD)
    pub n: bool,
    /// zero flag
    pub z: bool,
}

#[bitfield(u16)]
pub struct AFRegister {
    #[bits(8)]
    pub f: CPUFlags,
    pub a: u8,
}

#[bitfield(u16)]
pub struct BCRegister {
    pub c: u8,
    pub b: u8,
}

#[bitfield(u16)]
pub struct DERegister {
    pub e: u8,
    pub d: u8,
}

#[bitfield(u16)]
pub struct HLRegister {
    pub l: u8,
    pub h: u8,
}

#[bitfield(u8)]
pub struct IERegister {
    pub vblank: bool,
    pub lcd: bool,
    pub timer: bool,
    pub serial: bool,
    pub joypad: bool,
    #[bits(3)]
    _unused: u8,
}

#[bitfield(u8)]
pub struct IFRegister {
    pub vblank: bool,
    pub lcd: bool,
    pub timer: bool,
    pub serial: bool,
    pub joypad: bool,
    #[bits(3)]
    _unused: u8,
}

pub struct CPURegisters {
    pub af: AFRegister,
    pub bc: BCRegister,
    pub de: DERegister,
    pub hl: HLRegister,
    pub sp: u16,
    pub pc: u16,
    pub interrupt_enable: IERegister,
    pub interrupt_flag: IFRegister,
}

impl CPURegisters {
    pub fn get_r8(&self, r8: Register8) -> u8 {
        match r8 {
            Register8::A => self.af.a(),
            Register8::B => self.bc.b(),
            Register8::C => self.bc.c(),
            Register8::D => self.de.d(),
            Register8::E => self.de.e(),
            Register8::H => self.hl.h(),
            Register8::L => self.hl.l(),
        }
    }

    pub fn set_r8(&mut self, r8: Register8, value: u8) {
        match r8 {
            Register8::A => self.af.set_a(value),
            Register8::B => self.bc.set_b(value),
            Register8::C => self.bc.set_c(value),
            Register8::D => self.de.set_d(value),
            Register8::E => self.de.set_e(value),
            Register8::H => self.hl.set_h(value),
            Register8::L => self.hl.set_l(value),
        }
    }

    pub fn get_r16(&self, r16: Register16) -> u16 {
        match r16 {
            Register16::BC => self.bc.into_bits(),
            Register16::DE => self.de.into_bits(),
            Register16::HL => self.hl.into_bits(),
        }
    }

    pub fn set_r16(&mut self, r16: Register16, value: u16) {
        match r16 {
            Register16::BC => self.bc = BCRegister::from_bits(value),
            Register16::DE => self.de = DERegister::from_bits(value),
            Register16::HL => self.hl = HLRegister::from_bits(value),
        }
    }
}

#[derive(Debug, Clone, Copy, AsRefStr)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, AsRefStr, Clone, Copy)]
pub enum Register16 {
    BC,
    DE,
    HL,
}

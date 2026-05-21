use bitfield_struct::bitfield;

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
    c: u8,
    b: u8,
}

#[bitfield(u16)]
pub struct DERegister {
    e: u8,
    d: u8,
}

#[bitfield(u16)]
pub struct HLRegister {
    l: u8,
    h: u8,
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
}

#[derive(Debug, Clone, Copy)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum Register16 {
    BC,
    DE,
    HL,
}

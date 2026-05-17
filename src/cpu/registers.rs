use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct CPUFlags {
    #[bits(4)]
    _unused: u8,
    /// carry flag
    c: bool,
    /// half-carry flag (BCD)
    h: bool,
    /// subtraction flag (BCD)
    n: bool,
    /// zero flag
    z: bool,
}

#[bitfield(u16)]
pub struct AFRegister {
    #[bits(8)]
    f: CPUFlags,
    a: u8,
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

pub struct CPURegisters {
    pub af: AFRegister,
    pub bc: BCRegister,
    pub de: DERegister,
    pub hl: HLRegister,
    pub sp: u16,
    pub pc: u16,
}

#[derive(Debug)]
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

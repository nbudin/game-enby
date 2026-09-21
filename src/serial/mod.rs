use bitfield_struct::{bitenum, bitfield};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[bitenum]
pub enum SerialTransferClockSelect {
    #[fallback]
    External = 0,
    Internal = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[bitenum]
pub enum SerialTransferClockSpeed {
    #[fallback]
    NormalSpeed = 0,
    HighSpeed = 1,
}

#[bitfield(u8)]
pub struct SerialTransferControlRegister {
    #[bits(1)]
    pub clock_select: SerialTransferClockSelect,
    #[bits(1)]
    pub clock_speed: SerialTransferClockSpeed,
    #[bits(5)]
    _unused: u8,
    pub transfer_enable: bool,
}

pub struct SerialBus {
    pub data: u8,
    pub control_register: SerialTransferControlRegister,
}

impl SerialBus {
    pub fn new() -> SerialBus {
        SerialBus {
            data: 0,
            control_register: SerialTransferControlRegister::new(),
        }
    }
}

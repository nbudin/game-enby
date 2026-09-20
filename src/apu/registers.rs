use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct AudioMasterControlRegister {
    pub channel1_on: bool,
    pub channel2_on: bool,
    pub channel3_on: bool,
    pub channel4_on: bool,
    #[bits(3)]
    _unused: u8,
    pub audio_on: bool,
}

#[bitfield(u8)]
pub struct AudioPanningRegister {
    pub channel1_right: bool,
    pub channel2_right: bool,
    pub channel3_right: bool,
    pub channel4_right: bool,
    pub channel1_left: bool,
    pub channel2_left: bool,
    pub channel3_left: bool,
    pub channel4_left: bool,
}

#[bitfield(u8)]
pub struct AudioMasterVolumeVINPanningRegister {
    #[bits(3)]
    pub volume_right: u8,
    pub vin_right: bool,
    #[bits(3)]
    pub volume_left: u8,
    pub vin_left: bool,
}

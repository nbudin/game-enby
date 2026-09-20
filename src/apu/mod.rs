use crate::apu::registers::{
    AudioMasterControlRegister, AudioMasterVolumeVINPanningRegister, AudioPanningRegister,
};

pub mod registers;

pub struct APU {
    pub master_control: AudioMasterControlRegister,
    pub panning: AudioPanningRegister,
    pub master_volume_vin_panning: AudioMasterVolumeVINPanningRegister,
}

impl APU {
    pub fn new() -> APU {
        APU {
            master_control: AudioMasterControlRegister::new(),
            panning: AudioPanningRegister::new(),
            master_volume_vin_panning: AudioMasterVolumeVINPanningRegister::new(),
        }
    }
}

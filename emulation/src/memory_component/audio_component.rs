pub const AUDIO_MASTER_CONTROL_REGISTER: u16 = 0xff26u16;
pub const SOUND_PANNING_REGISTER: u16 = 0xff25u16;

pub enum AudioMasterControlFlag {
    Channel1Switch= 0b00000001,
    Channel2Switch= 0b00000010,
    Channel3Switch= 0b00000100,
    Channel4Switch= 0b00001000,
    MasterSwitch = 0b10000000,
}


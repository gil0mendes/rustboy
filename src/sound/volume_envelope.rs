use self::blip_buf::BlipBuf;

extern crate blip_buf;

pub struct VolumeEnvelope {
    enabled: bool,
    blip: BlipBuf
}

impl VolumeEnvelope {
    /// create a new VolumeEnvelope instance
    pub fn new(blip: BlipBuf) -> VolumeEnvelope {
        VolumeEnvelope {
            enabled: false,
            blip: blip
        }
    }

    /// check if the channel are enabled
    pub fn on(&self) -> bool {
        self.enabled
    }

    /// write a byte on sound channel
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            _ => { panic!("TODO VolumeEnvelope write address {:#x}", address) }
        }
    }
}

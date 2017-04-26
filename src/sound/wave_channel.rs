use self::blip_buf::BlipBuf;

extern crate blip_buf;

pub struct WaveChannel {
    enabled: bool,
    blip: BlipBuf
}

impl WaveChannel {
    /// create a new NoiseChannel instance
    pub fn new(blip: BlipBuf) -> WaveChannel {
        WaveChannel {
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
            _ => { panic!("TODO WaveChannel write address {:#x}", address) }
        }
    }
}

use self::blip_buf::BlipBuf;
use super::volume_envelope::VolumeEnvelope;

extern crate blip_buf;

pub struct NoiseChannel {
    enabled: bool,
    length: u8,
    new_length: u8,
    length_enabled: bool,
    volume_envelope: VolumeEnvelope,
    period: u32,
    shift_width: u8,
    state: u16,
    delay: u32,
    last_amp: i32,
    blip: BlipBuf
}

impl NoiseChannel {
    /// create a new NoiseChannel instance
    pub fn new(blip: BlipBuf) -> NoiseChannel {
        NoiseChannel {
            enabled: false,
            length: 0,
            new_length: 0,
            length_enabled: false,
            volume_envelope: VolumeEnvelope::new(),
            period: 2048,
            shift_width: 14,
            state: 1,
            delay: 0,
            last_amp: 0,
            blip
        }
    }

    /// check if the channel are enabled
    pub fn on(&self) -> bool {
        self.enabled
    }

    /// write a byte on sound channel
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x20 => self.new_length = 64 - (value & 0x3F),
            0x21 => (),
            0x22 => {
                self.shift_width = if value & 8 == 8 { 6 } else { 14 };
                let freq_div = match value & 7 {
                    0 => 8,
                    n => (n as u32 + 1) * 16,
                };
                self.period = freq_div << (value >> 4);
            },
            0x23 => {
                if value & 0x80 == 0x80 {
                    self.enabled = true;
                    self.length = self.new_length;
                    self.state = 0xFF;
                    self.delay = 0;
                }
            },
            _ => ()
        }

        // update volume envelope
        self.volume_envelope.write_byte(address, value);
    }
}

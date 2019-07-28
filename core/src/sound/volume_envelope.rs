use self::blip_buf::BlipBuf;

extern crate blip_buf;

pub struct VolumeEnvelope {
    period : u8,
    goes_up : bool,
    delay : u8,
    initial_volume : u8,
    volume : u8,
}

impl VolumeEnvelope {
    /// create a new VolumeEnvelope instance
    pub fn new() -> VolumeEnvelope {
        VolumeEnvelope {
            period: 0,
            goes_up: false,
            delay: 0,
            initial_volume: 0,
            volume: 0,
        }
    }

    /// write a byte on sound channel
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x12 | 0x17 | 0x21 => {
                self.period = value & 0x7;
                self.goes_up = value & 0x8 == 0x8;
                self.initial_volume = value >> 4;
                self.volume = self.initial_volume;
            },
            0x14 | 0x19 | 0x23 if value & 0x80 == 0x80 => {
                self.delay = self.period;
                self.volume = self.initial_volume;
            }
            _ => ()
        }
    }

    /// Step function for the volume envelop.
    pub fn step(&mut self) {
        // don't do nothing during the delay period
        if self.delay >  1 {
            self.delay -= 1;
        } else if self.delay == 1 {
            // reset delay
            self.delay = self.period;

            // raise the volume
            if self.goes_up && self.volume < 15 {
                self.volume += 1;
            } else if !self.goes_up && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }
}

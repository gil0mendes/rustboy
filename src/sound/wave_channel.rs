use self::blip_buf::BlipBuf;

extern crate blip_buf;

pub struct WaveChannel {
    enabled: bool,
    enabled_flag: bool,
    length: u16,
    new_length: u16,
    length_enabled: bool,
    frequency: u16,
    period: u32,
    last_amp: i32,
    delay: u32,
    volume_shift: u8,
    pub waveram: [u8; 32],
    current_wave: u8,
    blip: BlipBuf
}

impl WaveChannel {
    /// create a new NoiseChannel instance
    pub fn new(blip: BlipBuf) -> WaveChannel {
        WaveChannel {
            enabled: false,
            enabled_flag: false,
            length: 0,
            new_length: 0,
            length_enabled: false,
            frequency: 0,
            period: 2048,
            last_amp: 0,
            delay: 0,
            volume_shift: 0,
            waveram: [0; 32],
            current_wave: 0,
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
            0x1a => {
                self.enabled_flag = true;
                self.enabled = self.enabled && self.enabled_flag;
            }
            0x1b => self.new_length = 256 - (value as u16),
            0x1c => self.volume_shift = (value >> 5) & 0b11,
            0x1d => {
                self.frequency = (self.frequency & 0x0700) | (value as u16);
                self.calculate_period();
            }
            0x1e => {
                self.frequency = (self.frequency & 0x00FF) | (((value & 0b111) as u16) << 8);
                self.calculate_period();
                self.length_enabled = value & 0x40 == 0x40;
                if value & 0x80 == 0x80 && self.enabled_flag {
                    self.length = self.new_length;
                    self.enabled = true;
                    self.current_wave = 0;
                    self.delay = 0;
                }
            },
            0x30 | 0x3f => {
                self.waveram[(address as usize - 0xFF30) / 2] = value >> 4;
                self.waveram[(address as usize - 0xFF30) / 2 + 1] = value & 0xF;
            }
            _ => ()
        }
    }

    /// Compute the wave period.
    fn calculate_period(&mut self) {
        if self.frequency > 2048 {
            self.period = 0;
        } else {
            self.period = (2048 - self.frequency as u32) * 2;
        }
    }
}

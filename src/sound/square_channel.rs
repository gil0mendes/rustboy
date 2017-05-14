use self::blip_buf::BlipBuf;
use super::volume_envelope::VolumeEnvelope;

extern crate blip_buf;

pub struct SquareChannel {
    enabled: bool,
    duty : u8,
    phase : u8,
    length: u8,
    new_length: u8,
    length_enabled : bool,
    frequency: u16,
    period: u32,
    last_amp: i32,
    delay: u32,
    has_sweep : bool,
    sweep_frequency: u16,
    sweep_delay: u8,
    sweep_period: u8,
    sweep_shift: u8,
    sweep_frequency_increase: bool,
    volume_envelope: VolumeEnvelope,
    blip: BlipBuf,
}

impl SquareChannel {
    /// create a new SquareChannel instance.
    pub fn new(blip: BlipBuf, with_sweep: bool) -> SquareChannel {
        SquareChannel {
            enabled: false,
            duty: 1,
            phase: 1,
            length: 0,
            new_length: 0,
            length_enabled: false,
            frequency: 0,
            period: 2048,
            last_amp: 0,
            delay: 0,
            has_sweep: with_sweep,
            sweep_frequency: 0,
            sweep_delay: 0,
            sweep_period: 0,
            sweep_shift: 0,
            sweep_frequency_increase: false,
            volume_envelope: VolumeEnvelope::new(),
            blip: blip,
        }
    }

    /// check if the channel are enabled
    pub fn on(&self) -> bool {
        self.enabled
    }

    /// write a byte on sound channel
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x10 => {
                self.sweep_period = (value >> 4) & 0x7;
                self.sweep_shift = value & 0x7;
                self.sweep_frequency_increase = value & 0x8 == 0x8;
            },
            0x11 | 0x16 => {
                self.duty = value >> 6;
                self.new_length = 64 - (value & 0x3f);
            },
            0x13 | 0x18 => {
                panic!("TODO SquareChannel write address {:#x}", address)
            },
            0x14 | 0x19 => {
                panic!("TODO SquareChannel write address {:#x}", address)
            },
            _ => ()
        }

        // update the sound envelope
        self.volume_envelope.write_byte(address, value);
    }
}

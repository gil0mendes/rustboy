//! Game Boy sound emulation

use self::blip_buf::BlipBuf;
pub use self::player::CpalPlayer;
use self::wave_channel::WaveChannel;
use self::noise_channel::NoiseChannel;
use self::square_channel::SquareChannel;
use self::volume_envelope::VolumeEnvelope;

use io::io_map;

extern crate blip_buf;

mod player;
mod wave_channel;
mod noise_channel;
mod square_channel;
mod volume_envelope;

// --- Constants

const WAVE_PATTERN: [[i32; 8]; 4] = [[-1, -1, -1, -1, 1, -1, -1, -1], [-1, -1, -1, -1, 1, 1, -1, -1], [-1, -1, 1, 1, 1, 1, -1, -1], [1, 1, 1, 1, -1, -1, 1, 1]];
const CLOCKS_PER_SECOND: u32 = 1 << 22;
const OUTPUT_SAMPLE_COUNT: usize = 2000;

pub trait AudioPlayer: Send {
    fn play(&mut self, left_channel: &[f32], right_channel: &[f32]);
    fn samples_rate(&self) -> u32;
    fn underflowed(&self) -> bool;
}

/// create a new blipbuf instance with the correct clocks
fn create_blipbuf(samples_rate: u32) -> BlipBuf {
    let mut blipbuf = BlipBuf::new(samples_rate);
    blipbuf.set_rates(CLOCKS_PER_SECOND as f64, samples_rate as f64);
    blipbuf
}

/// Sound processing unit state
pub struct Sound {
    /// True if the sound circuit is enabled
    enabled: bool,
    /// Register data
    registerdata: [u8; 0x17],
    /// channel 1 (square channel)
    channel1: SquareChannel,
    /// channel 2 (square channel)
    channel2: SquareChannel,
    /// channel 3 (wave channel)
    channel3: WaveChannel,
    /// channel 4 (Noise channel)
    channel4: NoiseChannel,
    /// Player
    player: Box<AudioPlayer>,
    /// Left volume
    volume_left: u8,
    /// Right volume
    volume_right: u8,
}

impl Sound {
    /// create a new Sound instance
    pub fn new(player: Box<AudioPlayer>) -> Sound {
        // create a new blipbuf instance
        let blipbuf1 = create_blipbuf(player.samples_rate());
        let blipbuf2 = create_blipbuf(player.samples_rate());
        let blipbuf3 = create_blipbuf(player.samples_rate());
        let blipbuf4 = create_blipbuf(player.samples_rate());

        Sound {
            enabled: false,
            registerdata: [0x00; 0x17],
            channel1: SquareChannel::new(blipbuf1, true),
            channel2: SquareChannel::new(blipbuf2, false),
            channel3: WaveChannel::new(blipbuf3),
            channel4: NoiseChannel::new(blipbuf4),
            player,
            volume_left: 7,
            volume_right: 7,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            // data registers
            io_map::NR10 ... io_map::NR51 => self.registerdata[address as usize - 0x10],

            // enable and sound status
            io_map::NR52 => {
                (self.registerdata[address as usize - 0x10] & 0xf0)
                | (if self.channel1.on() { 1 } else { 0 })
                | (if self.channel2.on() { 2 } else { 0 })
                | (if self.channel3.on() { 4 } else { 0 })
                | (if self.channel4.on() { 8 } else { 0 })
            },

            // wave RAM
            io_map::NR3_RAM_START ... io_map::NR3_RAM_END => {
                (self.channel3.waveram[(address as usize - 0x30) / 2] << 4) |
                    self.channel3.waveram[(address as usize - 0x30) / 2 + 1]
            },

            _ => 0,
        }
    }

    /// write a value to the sound memory space
    pub fn write_byte(&mut self, address: u16, value: u8) {
        // The sound unity must be enabled to use all the address space
        if address != 0x26 && !self.enabled { return; }

        // registers address space
        if address >= 0x10 && address <= 0x26 {
            self.registerdata[address as usize - 0x10] = value;
        }

        match address {
            // Channel 1 address space
            0x10 ... 0x14 => self.channel1.write_byte(address, value),
            // Channel 2 address space
            0x16 ... 0x19 => self.channel2.write_byte(address, value),
            // Channel 3 address space
            0x1a ... 0x1e => self.channel3.write_byte(address, value),
            // Channel 4 address space
            0x20 ... 0x23 => self.channel4.write_byte(address, value),
            // Set volume
            0x24 => {
                self.volume_left = value & 0x7;
                self.volume_right = (value >> 4) & 0x7;
            }
            // Allow enable and disable the sound unity
            0x26 => self.enabled = value & 0x80 == 0x80,
            // Channel 3 address space echo
            0x30 ... 0x3f => self.channel3.write_byte(address, value),
            _ => (),
        }
    }
}

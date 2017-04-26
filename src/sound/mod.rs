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
    // Register data
    registerdata: [u8; 0x17],
    // channel 1 (square channel)
    channel1: SquareChannel,
    // channel 2 (square channel)
    channel2: SquareChannel,
    // channel 3 (wave channel)
    channel3: WaveChannel,
    // channel 4 (Noise channel)
    channel4: NoiseChannel,
    // Player
    player: Box<AudioPlayer>
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
            player: (),
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            // data registers
            io_map::NR10 ... io_map::NR51 => self.registerdata[address as usize],
            // enable and sound status
            io_map::NR52 => {
                (self.registerdata[address as usize] & 0xf0)
                    | (self.channel1.on() { 1 } else { 0 })
                | (self.channel2.on() { 2 } else { 0 })
                | (self.channel3.on() { 4 } else { 0 })
                | (self.channel4.on() { 8 } else { 0 })
            }

            _ => panic!("Sound handle read from address {:#x}", address),
        }
    }

    /// write a value to the sound memory space
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            _ => println!("Sound handle write to address {:#x}", address),
        }
    }
}

//! Game Boy sound emulation

use self::wave_channel::WaveChannel;
use self::noise_channel::NoiseChannel;
use self::square_channel::SquareChannel;
use self::volume_envelope::VolumeEnvelope;

mod wave_channel;
mod noise_channel;
mod square_channel;
mod volume_envelope;

/// Sound processing unit state
pub struct Sound {
  /// True if the sound circuit is enabled
  enabled: bool,
}

impl Sound {
  /// create a new Sound instance
  pub fn new () -> Sound {
    Sound {
      enabled: false
    }
  }

  pub fn read_byte(&self, address: u16) -> u8 {
    match address {
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
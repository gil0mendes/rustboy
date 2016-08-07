//! RAM emulation

use std::iter;

/// RAM image
pub struct Ram {
  data: Vec<u8>
}

impl Ram {
  
  /// Create a new RAM instance. The default RAM 
  /// values are undetermined so I just fill it 
  /// with garbage.
  pub fn new(size: usize) -> Ram {
    // create a new vector with random data
    let data = iter::repeat(0xca).take(size).collect();

    // return the new Ram instance
    Ram { data: data }
  }

  /// Get a byte from the Ram
  pub fn byte(&self, offset: u16) -> u8 { self.data[offset as usize] }

  /// Write a byte to the Ram
  pub fn set_byte(&mut self, offset: u16, val: u8) { self.data[offset as usize] = val; }

} 
//! Cartridge emulation.

// Each ROM bank is always 16KB
const ROM_BANK_SIZE: i32 = 16 * 1024;

// --------------------------------------------------------- [Cartridge]

/// Base state for all types of cartridges
pub struct Cartridge {
  // Cartridge ROM data
  rom: Vec<u8>,
  /// Current bank offset for the bank mapped at [0x4000, 0x7fff].
  /// This value is added to ROM register address when they're in 
  /// that range
  rom_offset: i32,
}

impl Cartridge {

  /// create a new Cartridge instance
  pub fn new(rom_buf: Vec<u8>) -> Cartridge {
    Cartridge {
      rom: rom_buf,
      rom_offset: 0
    }
  }

  /// read a byte from the cartridge memory
  pub fn read_byte(&self, offset: u16) -> u8 {
    let off = offset as i32;

    // read rom
    if off < ROM_BANK_SIZE {
      self.rom[off as usize]
    } else {
      self.rom[(self.rom_offset + off) as usize]
    }
  }

}
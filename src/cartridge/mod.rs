//! Cartridge emulation.

mod models;

// Each ROM bank is always 16KB
const ROM_BANK_SIZE: i32 = 16 * 1024;

mod offsets {
  //! Various offset values to access special memory location 
  //! within the ROM

  /// Cartridge type
  pub const TYPE: usize = 0x147;
}

// --------------------------------------------------------- [Cartridge]

/// Base state for all types of cartridges
pub struct Cartridge {
  // Cartridge ROM data
  rom: Vec<u8>,
  /// Cartridge RAM data
  ram: Vec<u8>,
  /// Total number of ROM banks in this cartridge
  rom_banks: u8,
  /// Current number of the rom bank mapped at [0x4000, 0x7fff]
  rom_bank: u8,
  /// Current bank offset for the bank mapped at [0x4000, 0x7fff].
  /// This value is added to ROM register address when they're in 
  /// that range
  rom_offset: i32,
  /// Current bank offset for the RAM
  ram_offset: u32,
  /// If `true` RAM is write protected
  ram_wp: bool,
  /// Certain cartridges allow banking either the RAM or ROM 
  /// depending on the value of this flag
  bank_ram: bool,
  /// Struct used to handle model specific functions
  model: models::Model
}

impl Cartridge {

  /// create a new Cartridge instance
  pub fn new(rom_buf: Vec<u8>) -> Cartridge {
    // determine the cartridge model
    let model = models::from_id(rom_buf[offsets::TYPE]);

    Cartridge {
      rom:        rom_buf,
      ram:        Vec::new(),
      // cartridge must have always at least two ROM banks
      rom_banks:  2,
      // default to bank 1 for bankable region
      rom_bank:   1,
      rom_offset: 0,
      ram_offset: 0,
      // by default RAM is write protected
      ram_wp:     true,
      bank_ram:   false,
      model:      model
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

  // ----------------------------------------------------------- [RAM]

  /// read a byte from the ram bank
  pub fn ram_byte(&self, offset: u16) -> u8 {
    // compute the address
    let address = self.ram_offset + offset as u32;

    // get byte from RAM
    (self.model.read_ram)(self, address)
  }

  /// Return the value of a RAM byte at absolute address
  fn ram_byte_absolute(&self, address: u32) -> u8 {
    *self.ram.get(address as usize).unwrap_or(&0)
  }

  fn ram_byte_absolute_mut(&mut self, addr: u32) -> Option<&mut u8> {
    self.ram.get_mut(addr as usize)
  }

  /// Enable or disable RAM write protect
  pub fn set_ram_wp(&mut self, wp: bool) {
    self.ram_wp = wp
  }

  /// Return the value of the `bank_ram` flag
  pub fn bank_ram(&self) -> bool {
    self.bank_ram
  }

  /// Set the value of the `bank_ram` flag
  pub fn set_bank_ram(&mut self, v: bool) {
    self.bank_ram = v
  }

  /// Set new RAM bank number
  pub fn set_ram_bank(&mut self, bank: u8) {
    // Bankable RAM is always 8KB per bank
    self.ram_offset = bank as u32 * 8 * 1024;
  }

  // ----------------------------------------------------------- [ROM]

  /// Retrieve the number of ROM banks in the cartridge
  pub fn rom_banks(&self) -> u8 {
    self.rom_banks
  }

  /// Retrieve current ROM bank number for the bankable 
  /// range at [0x4000, 0x7ffff]
  pub fn rom_bank(&self) -> u8 {
    self.rom_bank
  }

  /// Set new ROM bank number for the bankable range 
  /// at [0x4000, 0x7ffff]
  pub fn set_rom_bank(&mut self, bank: u8) {
    self.rom_bank = bank;
  }

  pub fn set_rom_offset(&mut self, offset: i32) {
    self.rom_offset = offset;
  }

  /// write a byte into ROM memory
  pub fn set_rom_byte(&mut self, offset: u16, value: u8) {
    (self.model.write_rom)(self, offset, value);
  }

}
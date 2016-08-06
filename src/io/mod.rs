#[derive(Debug)]
pub struct Interconnect {
  /// ROM bank #0 (16KB)
  rom: Vec<u8>,
  /// I/O ports
  io: Vec<u8>,
  /// Internal RAM
  hram: Vec<u8>,
  /// Interrupt Enable Register
  pub inte: u8,
  // Work RAM (8KB)
  wram: Vec<u8>
}

impl Interconnect {
  pub fn new(rom_buf: Vec<u8>) -> Interconnect {
    Interconnect {
      rom: rom_buf,
      io: vec![0x20; 0x7f],
      hram: vec![0x20; 0x7f],
      inte: 0x00,
      wram: vec![0x20; 0x2000]
    }
  }

  /// Reset the memory state
  pub fn reset(&mut self) {
    self.write_byte(0xff05, 0x00);
    self.write_byte(0xff06, 0x00);
    self.write_byte(0xff07, 0x00);
    self.write_byte(0xff10, 0x80);
    self.write_byte(0xff11, 0xbf);
    self.write_byte(0xff12, 0xf3);
    self.write_byte(0xff14, 0xbf);
    self.write_byte(0xff16, 0x3f);
    self.write_byte(0xff17, 0x00);
    self.write_byte(0xff19, 0xbf);
    self.write_byte(0xff1a, 0x7f);
    self.write_byte(0xff1b, 0xff);
    self.write_byte(0xff1c, 0x9f);
    self.write_byte(0xff1e, 0xbf);
    self.write_byte(0xff20, 0xff);
    self.write_byte(0xff21, 0x00);
    self.write_byte(0xff22, 0x00);
    self.write_byte(0xff23, 0xbf);
    self.write_byte(0xff24, 0x77);
    self.write_byte(0xff25, 0xf3);
    // TODO - set to 0xf0 on super game boy
    self.write_byte(0xff26, 0xf1);
    self.write_byte(0xff40, 0x91);
    self.write_byte(0xff42, 0x00);
    self.write_byte(0xff43, 0x00);
    self.write_byte(0xff45, 0x00);
    self.write_byte(0xff47, 0xfc);
    self.write_byte(0xff48, 0xff);
    self.write_byte(0xff49, 0xff);
    self.write_byte(0xff4a, 0x00);
    self.write_byte(0xff4b, 0x00);
    self.write_byte(0xffff, 0x00);
  }

  pub fn read_byte(&self, addr: u16) -> u8 {
    match addr {
      // Cartridge ROM
      0x0000 ... 0x3fff => self.rom[addr as usize],
      // Work RAM bank 0
      0xc000 ... 0xcfff => self.wram[addr as usize & 0x0fff],
      // TODO: this can be switchable bank 1-7 in GBC
      0xd000 ... 0xdfff => self.wram[addr as usize & 0x0fff],
      // TODO: I/O Ports
      0xff00 ... 0xff7f => self.io[addr as usize & 0x007f],
      _ => { panic!("Read from an unrecognized address: {:#x}", addr); }
    }
  }

  pub fn read_word(&self, addr: u16) -> u16 {
    (self.read_byte(addr) as u16) | ((self.read_byte(addr + 1) as u16) << 8)
  }

  pub fn write_byte(&mut self, addr: u16, value: u8) {
    match addr {
      0xc000 ... 0xcfff => self.wram[addr as usize & 0x0fff] = value,
      // TODO: this can be switchable bank 1-7 in GBC
      0xd000 ... 0xdfff => self.wram[addr as usize & 0x0fff] = value,
      // TODO: I/O Ports
      0xff00 ... 0xff7f => self.io[addr as usize & 0x007f] = value,
      // High RAM
      0xff80 ... 0xfffe => self.hram[addr as usize & 0x007f] = value,
      0xffff => self.inte = value,
      _ => { panic!("Write for an unrecognized address: {:#x}", addr); }
    }
  }

  /// write a word in memory
  ///
  /// # Arguments
  /// - addr 
  /// - value
  pub fn write_word(&mut self, addr: u16, value: u16) {
    self.write_byte(addr, (value & 0xff) as u8);
    self.write_byte(addr + 1, (value >> 8) as u8)
  }
}
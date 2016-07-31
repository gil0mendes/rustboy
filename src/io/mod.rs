#[derive(Debug)]
pub struct Interconnect {
  rom: Vec<u8>,
  io: Vec<u8>,
}

impl Interconnect {
  pub fn new(rom_buf: Vec<u8>) -> Interconnect {
    Interconnect {
      rom: rom_buf,
      io: vec![0x20; 0x7f],
    }
  }

  pub fn read_byte(&self, addr: u16) -> u8 {
    match addr {
      0x0000 ... 0x3fff => self.rom[addr as usize],
      _ => { panic!("Unrecognized address: {:#x}", addr); }
    }
  }

  pub fn read_word(&self, addr: u16) -> u16 {
    (self.read_byte(addr) as u16) | ((self.read_byte(addr + 1) as u16) << 8)
  }

  pub fn write_byte(&mut self, addr: u16, value: u8) {
    match addr {
      0xff00 ... 0xff7f => self.io[addr as usize] = value,
      _ => { panic!("Unrecognized address: {:#x}", addr); }
    }
  }
}
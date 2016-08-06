#[derive(Debug)]
pub struct GPU {
  line: u8
}

impl GPU {
  
  /// create a new GPU instance
  pub fn new() -> GPU {
    GPU {
      line: 0
    }
  }

  /// read one byte from the GPu structure
  pub fn read_byte(&self, address: u16) -> u8 {
    match address {
      0xff44 => self.line,
      _ => panic!("GPU can read {:#x} address", address)
    }
  }

  pub fn write_byte(&self, address: u16, value: u8) {
    match address {
      // LY - line (only read are supported)
      0xff44 => { },
      _ => panic!("GPU can write on {:#x} address", address),
    }
  }

}
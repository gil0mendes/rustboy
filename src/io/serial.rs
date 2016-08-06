// TODO: implement a callback handler (so we can 
// extend this on outside)

#[derive(Debug)]
pub struct Serial {
  // data
  data: u8,
  // control
  control: u8
}

impl Serial {

  /// create a new serial instance
  pub fn new() -> Serial {
    Serial {
      data: 0,
      control: 0
    }
  }

  /// read a byte from the serial
  pub fn read_byte(&self, address: u16) -> u8 {
    match address {
      0xff01 => self.data,
      0xff02 => self.control,
      _ => panic!("Serial does not handle address {:#x}", address)
    }
  }

  /// write a byte to the serial port
  pub fn write_byte(&mut self, address: u16, value: u8) {
    match address {
      0xff01 => self.data = value,
      0xff02 => self.control = value,
      _ => panic!("Serial does not handle address {:#x}", address)
    }
  }

}
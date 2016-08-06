#[derive(Debug)]
pub struct Sound {
}

impl Sound {
  /// create a new Sound instance
  pub fn new () -> Sound {
    Sound {

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
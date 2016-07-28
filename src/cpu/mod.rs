use self::registers::Registers;

mod registers;

#[derive(Debug)]
pub struct Cpu { 
  regs: Registers,
  halted: bool
}

impl Cpu {
  pub fn new(rom_buf: Vec<u8>) -> Cpu {
    Cpu {
      regs: Registers::new(),
      halted: false
    }
  }
}
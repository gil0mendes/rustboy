//! Game Boy CPU emulation

use self::registers::Registers;

mod registers;

/// CPU state
#[derive(Debug)]
pub struct Cpu { 
  /// CPU registers
  regs: Registers,
  /// CPU halted flag
  halted: bool
}

impl Cpu {
  /// Create a new Cpu instance and reset it
  pub fn new(rom_buf: Vec<u8>) -> Cpu {
    Cpu {
      regs: Registers::new(),
      halted: false
    }
  }
}
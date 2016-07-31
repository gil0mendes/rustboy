//! Game Boy CPU Registers emulation

/// CPU flags
#[derive(Debug)]
pub enum Flags {
  /// Zero FLag: set if the result of a match operation is 
  /// zero or two values compare equal
  Z = 0x10000000,
  /// Subtract Flag: set if the last math operation performed 
  /// a subtraction
  N = 0x01000000,
  /// Half Carry Flag: set if a carry occurred from the lower 
  /// nibble in the last math operation
  H = 0x00100000,
  /// Carry Flag: set if a carry occured during the last math 
  /// operation or if the first operand register compared smaller.
  C = 0x00010000,
}

/// CPU registers. They're 16 bit wide but some of them can 
/// be accessed as high and low byte.
#[derive(Debug,Clone,Copy)]
pub struct Registers {
  /// 8-bit `A` register (accumulator)
  pub a: u8,
  /// 8-bit `B` register
  pub b: u8,
  /// 8-bit `C` register
  pub c: u8,
  /// 8-bit `D` register
  pub d: u8,
  /// 8-bit `E` register
  pub e: u8,
  /// 8-bit `F` register (flags)
  f: u8,
  /// 8-bit `H` register
  pub h: u8,
  /// 8-bit `L` register
  pub l: u8,
  /// 16-bit (stack pointer)
  pub sp: u16,
  /// 16-bit (program counter)
  pub pc: u16,
}

impl Registers {
  /// create a new Registers instance and set the regs for 
  /// initial values after the internal ROM execute
  pub fn new() -> Registers {
    Registers {
      a: 0x01,
      b: 0x00,
      c: 0x13,
      d: 0x00,
      e: 0xd8,
      f: 0xb0,
      h: 0x01,
      l: 0x4d,
      sp: 0xfffe,
      pc: 0x100,
    }
  }

  /// create a new Register instance for GBC and set the 
  /// regs for initial value after the internal ROM execute
  pub fn new_gbc() -> Registers {
    Registers {
      a: 0x11,
      ..Registers::new()
    }
  }

  // -------------------------------------------------------------------- [Gets]

  /// Gets the af register value
  pub fn af(&self) -> u16 {
    ((self.a as u16) << 8) | ((self.f as u16) & 0xf0)
  }

  /// Gets the bc register value
  pub fn bc(&self) -> u16 {
    ((self.b as u16) << 8) | (self.c as u16)
  }

  /// Gets the de register value
  pub fn de(&self) -> u16 {
    ((self.d as u16) << 8) | (self.e as u16)
  }

  /// Gets the hl register value
  pub fn hl(&self) -> u16 {
    ((self.h as u16) << 8) | (self.l as u16)
  }

  // -------------------------------------------------------------------- [Sets]

  /// sets the af register value
  ///  
  /// # Arguments
  /// * `value` u8 - value 
  pub fn set_af(&mut self, value: u16) {
    self.a = (value >> 8) as u8;
    self.f = (value & 0x00ff) as u8;
  }

  /// sets the bc register value
  ///  
  /// # Arguments
  /// * `value` u8 - value 
  pub fn set_bc(&mut self, value: u16) {
    self.b = (value >> 8) as u8;
    self.c = value as u8;
  }

  /// sets the de register value
  ///  
  /// # Arguments
  /// * `value` u8 - value 
  pub fn set_de(&mut self, value: u16) {
    self.d = (value >> 8) as u8;
    self.e = value as u8;
  }

  /// sets the hl register value
  ///  
  /// # Arguments
  /// * `value` u8 - value 
  pub fn set_hl(&mut self, value: u16) {
    self.h = (value >> 8) as u8;
    self.l = value as u8;
  }

  /// sets the f register value
  ///  
  /// # Arguments
  /// * `value` u8 - value 
  pub fn set_f(&mut self, value: u8) {
    self.f = value & 0xf0;
  }

  // -------------------------------------------------------------------- [Flag]

  /// get flags state
  ///
  /// # Arguments
  /// * `flags` Flags - flags to read
  pub fn flag(&self, flags: Flags) -> bool {
    let mask = flags as u8;
    self.f & mask > 0
  }

  /// set the flags state
  ///
  /// # Arguments
  /// * `flags` Flags - flags to set
  /// * `set` bool       - if true active the flag
  pub fn set_flag(&mut self, flags: Flags, set: bool) {
    let mask = flags as u8;


    match set {
      true => self.f |= mask,
      false => self.f &= !mask,
    }

    self.f &= 0xf0;
  }
}

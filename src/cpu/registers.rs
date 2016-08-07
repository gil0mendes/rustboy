//! Game Boy CPU Registers emulation

/// CPU flags
#[derive(Debug,Copy,Clone)]
pub struct Flags {
  /// Carry Flag: set if a carry occured during the last math 
  /// operation or if the first operand register compared smaller.
  pub c: bool,
  /// Half Carry Flag: set if a carry occurred from the lower 
  /// nibble in the last math operation
  pub h: bool,
  /// Subtract Flag: set if the last math operation performed 
  /// a subtraction
  pub n: bool,
  /// Zero FLag: set if the result of a match operation is 
  /// zero or two values compare equal
  pub z: bool,
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
  /// 8-bit `H` register
  pub h: u8,
  /// 8-bit `L` register
  pub l: u8,
  /// 16-bit (stack pointer)
  pub sp: u16,
  /// 16-bit (program counter)
  pub pc: u16,
  /// CPU Flags
  pub flags: Flags,
}

impl Registers {
  /// create a new Registers instance and set the regs for 
  /// initial values after the internal ROM execute
  pub fn new() -> Registers {
    let mut instance = Registers {
      a: 0x00,
      b: 0x00,
      c: 0x00,
      d: 0x00,
      e: 0x00,
      h: 0x00,
      l: 0x00,
      sp: 0x0000,
      pc: 0x0000,
      flags: Flags {
        c: false,
        h: false,
        n: false,
        z: false
      }
    };

    // get value for f register
    instance.set_f(0xb0);

    // return instance
    instance
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
    ((self.a as u16) << 8) | ((self.f() as u16) & 0xf0)
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

  /// get hl value and decrement 
  pub fn hld(&mut self) -> u16 {
    let word = self.hl();
    self.set_hl(word - 1);
    word
  }

  /// get hl value and increment
  pub fn hli(&mut self) -> u16 {
    let word = self.hl();
    self.set_hl(word + 1);
    word
  }

  /// get value of f
  pub fn f(&self) -> u8 {
    let z = self.flags.z as u8;
    let n = self.flags.n as u8;
    let h = self.flags.h as u8;
    let c = self.flags.c as u8;

    (z << 7) | (n << 6) | ( h << 5) | (c << 4)
  }

  // -------------------------------------------------------------------- [Sets]

  /// sets the af register value
  ///  
  /// # Arguments
  /// * `value` u8 - value 
  pub fn set_af(&mut self, value: u16) {
    self.a = (value >> 8) as u8;

    // set f register
    self.set_f(value as u8);
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
  /// * `value` u16 - value 
  pub fn set_hl(&mut self, value: u16) {
    self.h = (value >> 8) as u8;
    self.l = value as u8;
  }

  /// sets the sp register value
  ///
  /// # Arguments
  /// * `value` u16 - value
  pub fn set_sp(&mut self, value: u16) {
    self.sp = value;
  }

  /// sets the f register value
  ///  
  /// # Arguments
  /// * `value` u8 - value 
  pub fn set_f(&mut self, value: u8) {
    self.flags.z = (value & (1 << 7)) != 0;
    self.flags.n = (value & (1 << 6)) != 0;
    self.flags.h = (value & (1 << 5)) != 0;
    self.flags.c = (value & (1 << 4)) != 0;
  }
}

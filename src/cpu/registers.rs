#[derive(Debug)]
enum CpuFlags {
  Z = 0x10000000,
  N = 0x01000000,
  H = 0x00100000,
  C = 0x00010000,
}

#[derive(Debug)]
pub struct Registers {
  /// 8-bit register (accumulator)
  pub a: u8,
  /// 8-bit register
  pub b: u8,
  /// 8-bit register
  pub c: u8,
  /// 8-bit register
  pub d: u8,
  /// 8-bit register
  pub e: u8,
  /// 8-bit register (flags)
  f: u8,
  /// 8-bit register
  pub h: u8,
  /// 8-bit register
  pub l: u8,
  /// 16-bit register (stack pointer)
  pub sp: u16,
  /// 16-bit register (program counter)
  pub pc: u16,
}

impl Registers {
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
  pub fn set_af(&mut self, value: u16) {
    self.a = (value >> 8) as u8;
    self.f = (value & 0x00ff) as u8;
  }

  /// sets the bc register value
  pub fn set_bc(&mut self, value: u16) {
    self.b = (value >> 8) as u8;
    self.c = value as u8;
  }

  /// sets the de register value
  pub fn set_de(&mut self, value: u16) {
    self.d = (value >> 8) as u8;
    self.e = value as u8;
  }

  /// sets the hl register value
  pub fn set_hl(&mut self, value: u16) {
    self.h = (value >> 8) as u8;
    self.l = value as u8;
  }

  /// sets the f register value
  pub fn set_f(&mut self, value: u8) {
    self.f = value & 0xf0;
  }

  // -------------------------------------------------------------------- [Flag]

  /// get flags state
  ///
  /// @param flags : CpuFlags   flags to read
  pub fn flag(&self, flags: CpuFlags) -> bool {
    let mask = flags as u8;
    self.f & mask > 0
  }

  /// set the flags state
  ///
  /// @param flags : CpuFlags   flags to set
  /// @param set : bool         if true all values will be reseted
  pub fn set_flag(&mut self, flags: CpuFlags, set: bool) {
    let mask = flags as u8;


    match set {
      true => self.f |= mask,
      false => self.f &= !mask,
    }

    self.f &= 0xf0;
  }
}
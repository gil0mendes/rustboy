#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
  VBank = 1 << 0,
  LCDState = 1 << 1,
  TimerOverflow = 1 << 2,
  SerialIOComplete = 1 << 3,
  JoyPad = 1 << 4,
}

impl Interrupt {
  pub fn from_u8(value: u8) -> Option<Interrupt> {
    use self::Interrupt::*;

    match value {
      1 => Some(VBank),
      2 => Some(LCDState),
      4 => Some(TimerOverflow),
      8 => Some(SerialIOComplete),
      16 => Some(JoyPad),
      _ => None,
    }
  }

  pub fn get_address(&self) -> u16 {
    match *self {
      Interrupt::VBank => 0x40,
      Interrupt::LCDState => 0x48,
      Interrupt::TimerOverflow => 0x50,
      Interrupt::SerialIOComplete => 0x58,
      Interrupt::JoyPad => 0x60,
    }
  }
}

bitflags!(
  pub struct InterruptType: u8 {
    const VBLANK = 1 << 0;
    const LCDSTAT = 1 << 1;
    const TIMER_OVERFLOW = 1 << 2;
    const SERIAL_IO_COMPLETED = 1 << 3;
    const JOYPAD = 1 << 4;
  }
);

#[derive(Debug)]
pub struct Irq {
  /// Interrupt Enable Register
  pub enabled: u8,
  /// Interrupt Flag
  pub flag: InterruptType,
}

impl Irq {
  pub fn new() -> Self {
    Self {
      enabled: 0x00,
      flag: InterruptType::empty(),
    }
  }

  pub fn get_interrupt_flag(&self) -> u8 {
    // Remove the unused bits
    const IF_UNUSED_MASK: u8 = (1 << 5) | (1 << 6) | (1 << 7);
    self.flag.bits | IF_UNUSED_MASK
  }

  pub fn set_interrupt_flag(&mut self, value: u8) {
    self.flag = InterruptType::from_bits_truncate(value);
  }

  pub fn get_interrupt_enabled(&self) -> u8 {
    self.enabled
  }

  pub fn set_interrupt_enabled(&mut self, value: u8) {
    self.enabled = value;
  }
}

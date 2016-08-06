#[derive(Debug)]
pub struct Timer {
  // DIV (Divider Register)
  divider: u8,
  // TIMA  (timer counter)
  counter: u8,
  // TMA (Timer Modulo)
  modulo: u8,
  // TAC (Timer Control)
  control: u8,
  // Interrupt
  pub interrupt: u8,
  // Internal Ticks
  internal_ticks: u32
}

impl Timer {
  
  /// create a new Timer instance
  pub fn new() -> Timer {
    Timer {
      divider: 0,
      counter: 0,
      modulo: 0,
      control: 0,
      interrupt: 0,
      internal_ticks: 0
    }
  }

  /// read a byte from the timer
  pub fn read_byte(&self, address: u16) -> u8 {
    match address {
      // divider 
      0xff04 => self.divider,
      // counter
      0xff05 => self.counter,
      // modulo
      0xff06 => self.modulo,
      // control
      0xff07 => self.control,
      _ => panic!("Timer does not handler read {:#x}", address),
    }
  }

  /// write a byte on timer
  pub fn write_byte(&mut self, address: u16, value: u8) {
    match address {
      // divider
      0xff04 => { self.divider = 0; }
      // counter
      0xff05 => { self.counter.wrapping_add(value); },
      // modulo
      0xff06 => { self.modulo.wrapping_add(value); },
      // control
      0xff07 => { self.control = value; },
      _ => panic!("Timer does not handler write {:#x}", address),
    }
  }

  /// get timer clock
  fn get_clock(&self) -> u8 {
    match self.control & 0b11 {
      // 4.096 KHz
      0b00 => 4,
      // 262.144 KHz
      0b01 => 262,
      // 65.536 KHz
      0b10 => 65,
      // 16.384 KHz
      0b11 => 16,
      // Invalid clock
      _ => panic!("Invalid clock"),
    }
  }

  /// is clock enable
  fn is_clock_enable(&self) -> bool { self.control & 0b100 == 1 }

  /// execute the timer cycle
  pub fn do_cycle(&mut self, ticks: u32)  {
    // we use the internal_ticks to check if we need
    // to increment the divider register
    self.internal_ticks += ticks;

    while self.internal_ticks >= 256 {
      self.divider = self.divider.wrapping_add(1);
      self.internal_ticks -= 256;
    }

    // is clock enable?
    if self.is_clock_enable() {
      // increment one to the counter
      self.counter.wrapping_add(1);

      // when the counter overflows generate an interrupt
      // and then are loaded with the contents of TMA 
      if self.counter == 0 {
        self.counter = self.modulo;
        self.interrupt |= 0b100;
      }
    }
  }

}
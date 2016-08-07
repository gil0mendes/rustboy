use self::gpu::GPU;
use self::timer::Timer;
use self::sound::Sound;
use self::serial::Serial;
use cartridge::Cartridge;

use self::ram::Ram;

mod map;
mod ram;
mod bootrom;

mod gpu;
mod timer;
mod sound;
mod serial;

pub struct Interconnect {
  /// Cartridge
  cartridge: Cartridge,
  /// I/O ports
  io: Vec<u8>,
  /// Internal RAM
  hram: Vec<u8>,
  /// Interrupt Enable Register
  pub inte: u8,
  /// Interrupt Flag 
  pub intf: u8,
  // Work RAM (8KB)
  wram: Ram,
  // GPU
  gpu: GPU,
  // Timer
  timer: Timer,
  // Sound
  sound: Sound,
  // Serial
  serial: Serial,
  // Bootrom is mapped
  bootrom: bool
}

impl Interconnect {
  pub fn new(cartridge: Cartridge) -> Interconnect {
    Interconnect {
      cartridge: cartridge,
      io: vec![0x20; 0x7f],
      hram: vec![0x20; 0x7f],
      inte: 0,
      intf: 0,
      wram: Ram::new(0x2000),
      gpu: GPU::new(),
      timer: Timer::new(),
      sound: Sound::new(),
      serial: Serial::new(),
      bootrom: true
    }
  }

  /// Reset the memory state
  /// TODO: implement the bootrom
  //pub fn reset(&mut self) {
  //  self.write_byte(0xff05, 0x00);
  //  self.write_byte(0xff06, 0x00);
  //  self.write_byte(0xff07, 0x00);
  //  self.write_byte(0xff10, 0x80);
  //  self.write_byte(0xff11, 0xbf);
  //  self.write_byte(0xff12, 0xf3);
  //  self.write_byte(0xff14, 0xbf);
  //  self.write_byte(0xff16, 0x3f);
  //  self.write_byte(0xff17, 0x00);
  //  self.write_byte(0xff19, 0xbf);
  //  self.write_byte(0xff1a, 0x7f);
  //  self.write_byte(0xff1b, 0xff);
  //  self.write_byte(0xff1c, 0x9f);
  //  self.write_byte(0xff1e, 0xbf);
  //  self.write_byte(0xff20, 0xff);
  //  self.write_byte(0xff21, 0x00);
  //  self.write_byte(0xff22, 0x00);
  //  self.write_byte(0xff23, 0xbf);
  //  self.write_byte(0xff24, 0x77);
  //  self.write_byte(0xff25, 0xf3);
  //  self.write_byte(0xff26, 0xf1);
  //  self.write_byte(0xff40, 0x91);
  //  self.write_byte(0xff42, 0x00);
  //  self.write_byte(0xff43, 0x00);
  //  self.write_byte(0xff45, 0x00);
  //  self.write_byte(0xff47, 0xfc);
  //  self.write_byte(0xff48, 0xff);
  //  self.write_byte(0xff49, 0xff);
  //  self.write_byte(0xff4a, 0x00);
  //  self.write_byte(0xff4b, 0x00);
  //  self.write_byte(0xffff, 0x00);
  //}

  pub fn do_cycle(&mut self, ticks: u32) {
    // TODO: this must use Game Boy Speed and VRAM ticks
    let cpu_ticks = ticks;

    // timer
    self.timer.do_cycle(cpu_ticks);
    self.inte |= self.timer.interrupt;
    self.timer.interrupt = 0;
  }

  /// read a byte from the interconnect
  pub fn read_byte(&self, address: u16) -> u8 {

    // ROM
    if let Some(off) = map::in_range(address, map::ROM) {
      // bootrom is still mapped, read from it
      if self.bootrom && off < 0x100 { 
        return bootrom::BOOTROM[off as usize];
      }

      // read a byte from the cartridge
      return self.cartridge.read_byte(off);
    }

    // RAM bank
    if let Some(off) = map::in_range(address, map::RAM_BANK) {
      // TODO
    }



    // Work RAM bank 0
    //0xc000 ... 0xcfff =>  self.wram[address as usize & 0x0fff],

    // --- Old Implementation

    match address {
      // TODO: this can be switchable bank 1-7 in GBC
      0xd000 ... 0xdfff => self.wram.byte(address & 0x0fff),
      // Keypad
      0xff00 => panic!("TODO: read keypad"),
      // Serial
      0xff01 ... 0xff02 => self.serial.read_byte(address),
      // Time
      0xff04 ... 0xff07 => self.timer.read_byte(address),
      // Interrupt Flags
      0xff0f => self.intf,
      // Sound
      0xff10 ... 0xff3f => panic!("TODO: read sounds"),
      // GPU
      0xff40 ... 0xff4b => self.gpu.read_byte(address),
      // Infrared (Implementation don't needed)
      0xff56 => { 0 },
      // High RAM
      0xff80 ... 0xfffe => self.hram[address as usize & 0x007f],
      // IE (Interrupt Enable)
      0xffff => self.inte,
      // invalid address
      _ => { panic!("Read from an unrecognized address: {:#x}", address); }
    }
  }

  /// read a word from the interconnect
  pub fn read_word(&self, addr: u16) -> u16 {
    (self.read_byte(addr) as u16) | ((self.read_byte(addr + 1) as u16) << 8)
  }

  /// write a byte to the interconnect
  pub fn write_byte(&mut self, address: u16, value: u8) {
    match address {
      0xc000 ... 0xcfff => self.wram.set_byte(address & 0x0fff, value),
      // TODO: this can be switchable bank 1-7 in GBC
      0xd000 ... 0xdfff => self.wram.set_byte(address & 0x0fff, value),
      // Serial
      0xff01 ... 0xff02 => self.serial.write_byte(address, value),
      // Timer
      0xff04 ... 0xff07 => self.timer.write_byte(address, value),
      // Interrupt Flags
      0xff0f => self.intf = value,
      // Sound
      0xff10 ... 0xff3f => self.sound.write_byte(address, value),
      // GPU
      0xff40 ... 0xff4b => self.gpu.write_byte(address, value),
      // Infrared (Implementation don't needed)
      0xff56 => { },
      // High RAM
      0xff80 ... 0xfffe => self.hram[address as usize & 0x007f] = value,
      // Interrupt Enable
      0xffff => self.inte = value,
      _ => { panic!("Write for an unrecognized address: {:#x}", address); }
    }
  }

  /// write a word in memory
  ///
  /// # Arguments
  /// - addr 
  /// - value
  pub fn write_word(&mut self, addr: u16, value: u16) {
    self.write_byte(addr, (value & 0xff) as u8);
    self.write_byte(addr + 1, (value >> 8) as u8)
  }
}
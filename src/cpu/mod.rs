//! Game Boy CPU emulation

use self::registers::{Registers, Flags};
use super::io::Interconnect;

mod registers;

/// CPU state
#[derive(Debug)]
pub struct Cpu { 
  /// CPU registers
  regs: Registers,
  /// Interconnect
  interconnect: Interconnect,
  /// CPU halted flag
  halted: bool
}

impl Cpu {

  /// Create a new Cpu instance and reset it
  pub fn new(inter: Interconnect) -> Cpu {
    Cpu {
      regs: Registers::new(),
      halted: false,
      interconnect: inter
    }
  }

  /// create a new cpu instance for GBC and reset it
  pub fn new_gbc(inter: Interconnect) -> Cpu {
    Cpu {
      regs: Registers::new_gbc(),
      ..Cpu::new(inter)
    }
  }

  fn fetch_byte(&mut self) -> u8 {
    let byte = self.interconnect.read_byte(self.regs.pc);
    self.regs.pc += 1;
    byte
  }

  fn fetch_word(&mut self) -> u16 {
    let word = self.interconnect.read_word(self.regs.pc);
    self.regs.pc += 2;
    word
  }

  pub fn run(&mut self) {
    loop {
      println!("{:?}", self.regs);

      // process the next instruction
      let ticks = self.process_next_insctruction();
    }
  }

  fn process_next_insctruction(&mut self) -> u8 {
    // fetch a byte from the PC address
      let opcode = self.fetch_byte();

      // get current regs state
      let regs = self.regs;

      match opcode {
        // NOP
        0x00 => { 1 },
        0x01 => { let word = self.fetch_word(); self.regs.set_bc(word); 3 },
        // JP (HL)
        0x18 => {
          self.regs.pc = regs.pc + (self.fetch_byte() as u16);
          1
        },
        // JR cc,n
        0x28 => {
          let n = self.fetch_byte();

          if regs.flag(Flags::Z) {
            if regs.flag(Flags::C) {
              self.regs.pc = regs.pc + (n as u16);
              3
            } else {
              2
            }
          } else {
            2
          }
        },
        // LD A,n
        0x3e => { self.regs.a = self.fetch_byte(); 2 },
        // XOR
        0xaf => {
          self.alu_xor(regs.a);
          1
        },
        // JP nn
        0xc3 => { 
          self.regs.pc = self.fetch_word();
          3
        },
        // LDH (n),a
        0xe0 => {
          let byte = self.fetch_byte() as u16;
          self.interconnect.write_byte(0xff00 + byte, regs.a);
          3
        },
        // CP n
        0xfe => {
          // gets n
          let byte = self.fetch_byte();

          // make the logic calculation
          self.alu_cp(byte);

          2
        },
        _ => { panic!("Unrecognized opcode: {:#x}", opcode); },
      }
  }

  fn alu_sub(&mut self, byte: u8, usec: bool) {
    // get carry
    let c = if usec && self.regs.flag(Flags::C) { 1 } else { 0 };

    let a = self.regs.a;

    let result = a.wrapping_sub(byte).wrapping_sub(c);

    // set flags
    self.regs.set_flag(Flags::Z, result == 0);
    self.regs.set_flag(Flags::N, true);
    self.regs.set_flag(Flags::H, (a & 0x0f) < (result & 0x0f) + c);
    self.regs.set_flag(Flags::C, (a as u16) < (result as u16) + (c as u16));

    self.regs.a = result;
  }

  fn alu_cp(&mut self, byte: u8) {
    let old_a = self.regs.a;
    self.alu_sub(byte, false);
    self.regs.a = old_a;
  }

  fn alu_xor(&mut self, byte: u8) {
    let result = self.regs.a ^ byte;
    self.regs.set_flag(Flags::Z, result == 0);
    self.regs.set_flag(Flags::N, false);
    self.regs.set_flag(Flags::H, false);
    self.regs.set_flag(Flags::C, false);
    self.regs.a = result;
  }
}
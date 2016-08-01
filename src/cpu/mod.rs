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
        // LD (BC),nn
        0x01 => { let word = self.fetch_word(); self.regs.set_bc(word); 3 },
        // LD (BC),A
        0x02 => { self.interconnect.write_byte(regs.bc(), regs.a); 2 },
        // LD B,n
        0x06 => { self.regs.b = self.fetch_byte(); 2 },
        // LD DE,nn
        0x11 => { let word = self.fetch_word(); self.regs.set_de(word); 3 },
        // LD A, (BC)
        0x0a => { self.regs.a = self.interconnect.read_byte(regs.bc()); 2 },
        // LD (DE),A
        0x12 => { self.interconnect.write_byte(regs.de(), regs.a); 2 },
        // JP (HL)
        0x18 => { self.regs.pc = regs.pc + (self.fetch_byte() as u16); 1 },
        // LD C,n
        0x0e => { self.regs.c = self.fetch_byte(); 2 },
        // LD D,n
        0x16 => { self.regs.d = self.fetch_byte(); 2 },
        // LD A,(DE)
        0x1a => { self.regs.a = self.interconnect.read_byte(regs.de()); 2 },
        // LD E,n
        0x1e => { self.regs.e = self.fetch_byte(); 2 },
        // LD HL,nn
        0x21 => { let word = self.fetch_word(); self.regs.set_hl(word); 3 },
        // LD (HLI),A
        0x22 => { self.interconnect.write_byte(self.regs.hli(), regs.a); 2 },
        // LD H,n
        0x26 => { self.regs.h = self.fetch_byte(); 2 },
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
        // LDI A,(HL)
        0x2a => { self.regs.a = self.interconnect.read_byte(self.regs.hli()); 2 },
        // LD L,n
        0x2e => { self.regs.l = self.fetch_byte(); 2 },
        // LD SP,nn
        0x31 => { let word = self.fetch_word(); self.regs.set_sp(word); 3 },
        // LD (HLD)
        0x32 => { self.interconnect.write_byte(self.regs.hld(), regs.a); 2 },
        // LD (HL),n
        0x36 => { let byte = self.fetch_byte(); self.interconnect.write_byte(regs.hl(), byte); 3 },
        // LD A,(HLD)
        0x3a => { self.regs.a = self.interconnect.read_byte(self.regs.hld()); 2 },
        // LD A,n
        0x3e => { self.regs.a = self.fetch_byte(); 2 },
        // LD B,B
        0x40 => { 1 },
        // LD B,C
        0x41 => { self.regs.b = regs.c; 1 },
        // LD B,C
        0x42 => { self.regs.b = regs.c; 1 },
        // LD B,E
        0x43 => { self.regs.b = regs.e; 1 },
        // LD B,H
        0x44 => { self.regs.b = regs.h; 1 },
        // LD B,L
        0x45 => { self.regs.b = regs.l; 1 },
        // LD B,(HL)
        0x46 => { self.regs.b = self.interconnect.read_byte(regs.hl()); 2 },
        // LD B,A
        0x47 => { self.regs.b = regs.a; 1 },
        // LD C,B
        0x48 => { self.regs.c = regs.b; 1 },
        // LD C,C
        0x49 => { 1 },
        // LD C,D
        0x4a => { self.regs.c = regs.d; 1 },
        // LD C,E
        0x4b => { self.regs.c = regs.e; 1 },
        // LD C,H
        0x4c => { self.regs.c = regs.h; 1 },
        // LD C,L
        0x4d => { self.regs.c = regs.l; 1 },
        // LD C,(HL)
        0x4e => { self.regs.c = self.interconnect.read_byte(regs.hl()); 2 },
        // LD C,A
        0x4f => { self.regs.c = regs.a; 1 },
        // LD D,B
        0x50 => { self.regs.d = regs.b; 1 },
        // LD D,C
        0x51 => { self.regs.d = regs.c; 1 },
        // LD D,D
        0x52 => { 1 },
        // LD D,E
        0x53 => { self.regs.d = regs.e; 1 },
        // LD D,H
        0x54 => { self.regs.d = regs.h; 1 },
        // LD D,L
        0x55 => { self.regs.d = regs.l; 1 },
        // LD D,(HL)
        0x56 => { self.regs.d = self.interconnect.read_byte(regs.hl()); 2 },
        // LD D,A
        0x57 => { self.regs.d = regs.a; 1 },
        // LD E,B
        0x58 => { self.regs.e = regs.b; 1 },
        // LD E,C
        0x59 => { self.regs.e = regs.c; 1 },
        // LD E,D
        0x5a => { self.regs.e = regs.d; 1 },
        // LD E,E
        0x5b => { 1 },
        // LD E,H
        0x5c => { self.regs.e = regs.h; 1 },
        // LD E,L
        0x5d => { self.regs.e = regs.l; 1 },
        // LD E,(HL)
        0x5e => { self.regs.e = self.interconnect.read_byte(regs.hl()); 2 },
        // LD E,A
        0x5f => { self.regs.e = regs.a; 1 },
        // LD H,B
        0x60 => { self.regs.h = regs.b; 1 },
        // LD H,C
        0x61 => { self.regs.h = regs.c; 1 },
        // LD H,D
        0x62 => { self.regs.h = regs.d; 1 },
        // LD H,E
        0x63 => { self.regs.h = regs.e; 1 },
        // LD H,H
        0x64 => { 1 },
        // LD H,L
        0x65 => { self.regs.h = regs.l; 1 },
        // LD H,(HL)
        0x66 => { self.regs.h = self.interconnect.read_byte(regs.hl()); 2 },
        // LD H,A
        0x67 => { self.regs.h = regs.a; 1 },
        // LD L,B
        0x68 => { self.regs.l = regs.b; 1 },
        // LD L,C
        0x69 => { self.regs.l = regs.c; 1 },
        // LD L,D
        0x6a => { self.regs.l = regs.d; 1 },
        // LD L,E
        0x6b => { self.regs.l = regs.e; 1 },
        // LD L,H
        0x6c => { self.regs.l = regs.h; 1 },
        // LD L,L
        0x6d => { 1 },
        // LD L,(HL)
        0x6e => { self.regs.l = self.interconnect.read_byte(regs.hl()); 2 },
        // LD L,A
        0x6f => { self.regs.l = regs.a; 1 },
        // LD (HL),B
        0x70 => { self.interconnect.write_byte(regs.hl(), regs.b); 2 },
        // LD (HL),C
        0x71 => { self.interconnect.write_byte(regs.hl(), regs.c); 2 },
        // LD (HL),D
        0x72 => { self.interconnect.write_byte(regs.hl(), regs.d); 2 },
        // LD (HL),E
        0x73 => { self.interconnect.write_byte(regs.hl(), regs.e); 2 },
        // LD (HL),H
        0x74 => { self.interconnect.write_byte(regs.hl(), regs.h); 2 },
        // LD (HL),L
        0x75 => { self.interconnect.write_byte(regs.hl(), regs.l); 2 },
        // LD (HL),A
        0x77 => { self.interconnect.write_byte(regs.hl(), regs.a); 2 },
        // LD A,B
        0x78 => { self.regs.a = regs.b; 1 },
        // LD A,C
        0x79 => { self.regs.a = regs.c; 1 },
        // LD A,D
        0x7a => { self.regs.a = regs.d; 1 },
        // LD A,E
        0x7b => { self.regs.a = regs.e; 1 },
        // LD A,H
        0x7c => { self.regs.a = regs.h; 1 },
        // LD A,L
        0x7d => { self.regs.a = regs.l; 1 },
        // LD A,(HL)
        0x7e => { self.regs.a = self.interconnect.read_byte(regs.hl()); 2 }
        // LD A,A
        0x7f => { 1 },
        // XOR
        0xaf => { self.alu_xor(regs.a); 1 },
        // JP nn
        0xc3 => { self.regs.pc = self.fetch_word(); 3 },
        // LDH (n),a
        0xe0 => { let byte = self.fetch_byte() as u16; self.interconnect.write_byte(0xff00 + byte as u16, regs.a); 3 },
        // LD (0xff00+C),A
        0xe2 => { self.interconnect.write_byte(0xff00 + regs.c as u16, regs.a); 2 },
        // LD (nn),A
        0xea => { let word = self.fetch_word(); self.interconnect.write_byte(word, regs.a); 3 },
        // LDH A,(n)
        0xf0 => { let byte = self.fetch_byte(); self.regs.a = self.interconnect.read_byte(0xff00 + byte as u16); 3 },
        // LD A,(0xff00+C)
        0xf2 => { let byte = self.interconnect.read_byte(0xff00 + regs.c as u16); self.regs.a = byte; 2 },
        // LDHL SP,n
        // TODO: find more information about this opcode
        0xf8 => { 
          let addr = regs.sp() + self.fetch_byte() as u16;
          self.regs.set_hl(addr);
          self.regs.set_flag(Flags::Z, false);
          self.regs.set_flag(Flags::N, false);

        },
        // LD SP,HL
        0xf9 => { self.regs.set_sp(regs.hl()); 2 },
        // LD A,(nn)
        0xfa => { let word = self.fetch_word(); self.regs.a = self.interconnect.read_byte(word); 3 },
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
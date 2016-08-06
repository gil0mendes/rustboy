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

  /// push a value to stack
  fn stack_push(&mut self, value: u16) {
      self.interconnect.write_word(self.regs.sp, value);
      self.regs.sp -= 2
  }

  /// pop a value from stack
  fn stack_pop(&mut self) -> u16 {
      let value = self.interconnect.read_word(self.regs.sp);
      self.regs.sp += 2;
      value
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
        // LD (nn),SP
        0x08 => { let word = self.fetch_word(); self.interconnect.write_word(word, regs.sp); 5},
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
        // ADD A,B
        0x80 => { self.alu_add(regs.b, false); 1 },
        // ADD A,C
        0x81 => { self.alu_add(regs.c, false); 1 },
        // ADD A,D
        0x82 => { self.alu_add(regs.d, false); 1 },
        // ADD A,E
        0x83 => { self.alu_add(regs.e, false); 1 },
        // ADD A,H
        0x84 => { self.alu_add(regs.h, false); 1 },
        // ADD A,L
        0x85 => { self.alu_add(regs.l, false); 1 },
        // ADD A,(HL)
        0x86 => { let value = self.interconnect.read_byte(regs.hl()); self.alu_add(value, true); 2 },
        // ADD A,A
        0x87 => { self.alu_add(regs.a, false); 1 },
        // ADC A,B
        0x88 => { self.alu_add(regs.b, true); 1 },
        // ADC A,C
        0x89 => { self.alu_add(regs.c, true); 1 },
        // ADC A,D
        0x8a => { self.alu_add(regs.d, true); 1 },
        // ADC A,E
        0x8b => { self.alu_add(regs.e, true); 1 },
        // ADC A,H
        0x8c => { self.alu_add(regs.h, true); 1 },
        // ADC A,L
        0x8d => { self.alu_add(regs.l, true); 1 },
        // ADC A,(HL)
        0x8e => { let value = self.interconnect.read_byte(regs.hl()); self.alu_add(value, true); 2 },
        // ADC A,A
        0x8f => { self.alu_add(regs.a, true); 1 },
        // SUB B
        0x90 => { self.alu_sub(regs.b, false); 1 },
        // SUB C
        0x91 => { self.alu_sub(regs.c, false); 1 },
        // SUB D
        0x92 => { self.alu_sub(regs.d, false); 1 },
        // SUB E
        0x93 => { self.alu_sub(regs.e, false); 1 },
        // SUB H
        0x94 => { self.alu_sub(regs.h, false); 1 },
        // SUB L
        0x95 => { self.alu_sub(regs.l, false); 1 },
        // SUB (HL)
        0x96 => { let value = self.interconnect.read_byte(regs.hl()); self.alu_sub(value, false); 2 },
        // SBC B
        0x98 => { self.alu_sub(regs.b, true); 1 },
        // SBC C
        0x99 => { self.alu_sub(regs.c, true); 1 },
        // SBC D
        0x9a => { self.alu_sub(regs.d, true); 1 },
        // SBC E
        0x9b => { self.alu_sub(regs.e, true); 1 },
        // SBC H
        0x9c => { self.alu_sub(regs.h, true); 1 },
        // SBC L
        0x9d => { self.alu_sub(regs.l, true); 1 },
        // SBC (HL)
        0x9e => { let value = self.interconnect.read_byte(regs.hl()); self.alu_sub(value, true); 2 },
        // SUB A
        0x97 => { self.alu_sub(regs.a, false); 1 },
        // SBC A
        0x9f => { self.alu_sub(regs.a, true); 1 },
        // AND B
        0xa0 => { self.alu_and(regs.b); 1 },
        // AND C
        0xa1 => { self.alu_and(regs.c); 1 },
        // AND D
        0xa2 => { self.alu_and(regs.d); 1 },
        // AND E
        0xa3 => { self.alu_and(regs.e); 1 },
        // AND H
        0xa4 => { self.alu_and(regs.h); 1 },
        // AND L
        0xa5 => { self.alu_and(regs.l); 1 },
        // AND (HL)
        0xa6 => { let value = self.interconnect.read_byte(regs.hl()); self.alu_and(value); 1 },
        // AND B
        0xa8 => { self.alu_and(regs.b); 1 },
        // AND C
        0xa9 => { self.alu_and(regs.c); 1 },
        // AND D
        0xaa => { self.alu_and(regs.d); 1 },
        // AND E
        0xab => { self.alu_and(regs.e); 1 },
        // AND H
        0xac => { self.alu_and(regs.h); 1 },
        // AND L
        0xad => { self.alu_and(regs.l); 1 },
        // AND (HL)
        0xae => { let value = self.interconnect.read_byte(regs.hl()); self.alu_and(value); 2 },
        // AND A
        0xa7 => { self.alu_and(regs.a); 1 },
        // XOR
        0xaf => { self.alu_xor(regs.a); 1 },
        // OR B
        0xb0 => { self.alu_or(regs.b); 1 },
        // OR C
        0xb1 => { self.alu_or(regs.c); 1 },
        // OR D
        0xb2 => { self.alu_or(regs.d); 1 },
        // OR E
        0xb3 => { self.alu_or(regs.e); 1 },
        // OR H
        0xb4 => { self.alu_or(regs.h); 1 },
        // OR L
        0xb5 => { self.alu_or(regs.l); 1 },
        // OR (HL)
        0xb6 => { let value = self.interconnect.read_byte(regs.hl()); self.alu_or(value); 2 },
        // OR A
        0xb7 => { self.alu_or(regs.a); 1 },
        // CP B
        0xb8 => { self.alu_cp(regs.a); 1 },
        // CP C
        0xb9 => { self.alu_cp(regs.c); 1 },
        // CP D
        0xba => { self.alu_cp(regs.d); 1 },
        // CP E
        0xbb => { self.alu_cp(regs.e); 1 },
        // CP H
        0xbc => { self.alu_cp(regs.h); 1 },
        // CP L
        0xbd => { self.alu_cp(regs.l); 1 },
        // CP (HL)
        0xbe => { let value = self.interconnect.read_byte(regs.hl()); self.alu_cp(value); 1 },
        // CP A
        0xbf => { self.alu_cp(regs.a); 1 },
        // POP BC
        0xc1 => { let value = self.stack_pop(); self.regs.set_bc(value); 3 },
        // JP nn
        0xc3 => { self.regs.pc = self.fetch_word(); 3 },
        // PUSH BC
        0xc5 => { self.stack_push(regs.bc()); 3 },
        // ADD A,#
        0xc6 => { let value = self.fetch_byte(); self.alu_add(value, false); 2 },
        // ADC A,#
        0xce => { let value = self.fetch_byte(); self.alu_add(value, true); 2 },
        // POP DE
        0xd1 => { let value = self.stack_pop(); self.regs.set_de(value); 3 },
        // PUSH DE
        0xd5 => { self.stack_push(regs.de()); 3 },
        // SUB #
        0xd6 => { let value = self.fetch_byte(); self.alu_sub(value, false); 2 },
        // SUC #
        0xde => { let value = self.fetch_byte(); self.alu_sub(value, true); 2 },
        // LDH (n),a
        0xe0 => { let byte = self.fetch_byte() as u16; self.interconnect.write_byte(0xff00 + byte as u16, regs.a); 3 },
        // POP HL
        0xe1 => { let value = self.stack_pop(); self.regs.set_hl(value); 3 },
        // LD (0xff00+C),A
        0xe2 => { self.interconnect.write_byte(0xff00 + regs.c as u16, regs.a); 2 },
        // PUSH hl
        0xe5 => { self.stack_push(regs.hl()); 3 },
        // AND #
        0xe6 => { let value = self.fetch_byte(); self.alu_and(value); 1 },
        // LD (nn),A
        0xea => { let word = self.fetch_word(); self.interconnect.write_byte(word, regs.a); 3 },
        // AND #
        0xee => { let value = self.fetch_byte(); self.alu_and(value); 2 },
        // LDH A,(n)
        0xf0 => { let byte = self.fetch_byte(); self.regs.a = self.interconnect.read_byte(0xff00 + byte as u16); 3 },
        // POP AF
        0xf1 => { let value = self.stack_pop(); self.regs.set_af(value); 3 },
        // LD A,(0xff00+C)
        0xf2 => { let byte = self.interconnect.read_byte(0xff00 + regs.c as u16); self.regs.a = byte; 2 },
        // PUSH AF
        0xf5 => { self.stack_push(regs.af()); 4 },
        // OR #
        0xf6 => { let value = self.fetch_byte(); self.alu_or(value); 2 },
        // LDHL SP,n
        0xf8 => { let result = self.alu_add16imm(regs.sp); self.regs.set_hl(result); 2 },
        // LD SP,HL
        0xf9 => { self.regs.set_sp(regs.hl()); 2 },
        // LD A,(nn)
        0xfa => { let word = self.fetch_word(); self.regs.a = self.interconnect.read_byte(word); 3 },
        // CP n
        0xfe => { let byte = self.fetch_byte(); self.alu_cp(byte); 2 },
        _ => { panic!("Unrecognized opcode: {:#x}", opcode); },
      }
  }

  fn alu_or(&mut self, value: u8) {
    // make the logical or operation
    let result = self.regs.a | value;

    // update reg A value
    self.regs.a = result;

    // update CPU flags
    self.regs.set_flag(Flags::Z, result == 0);
    self.regs.set_flag(Flags::N, false);
    self.regs.set_flag(Flags::H, false);
    self.regs.set_flag(Flags::C, false);
  }

  /// make a logical and with the reg A
  fn alu_and(&mut self, value: u8) {
    // make the logical and operation
    let result = self.regs.a & value;

    // update reg A value
    self.regs.a = result;

    // update CPU flags
    self.regs.set_flag(Flags::Z, result == 0);
    self.regs.set_flag(Flags::N, false);
    self.regs.set_flag(Flags::H, true);
    self.regs.set_flag(Flags::C, false);
  }

  fn alu_add16imm(&mut self, sp: u16) -> u16 {
    let byte = self.fetch_byte() as i8 as u16;
    
    self.regs.set_flag(Flags::Z, false);
    self.regs.set_flag(Flags::N, false);
    self.regs.set_flag(Flags::H, byte & 0x000f > 0x000f);
    self.regs.set_flag(Flags::C, byte & 0x00ff > 0x00ff);

    sp.wrapping_add(byte)
  }

  /// add a value to reg A
  fn alu_add(&mut self, value: u8, usec: bool) {
    // get carry (if is to use)
    let carry = if usec && self.regs.flag(Flags::C) { 1 } else { 0 };
    
    // get reg a value
    let a = self.regs.a;

    // compute the new value
    let result = a.wrapping_add(value).wrapping_add(carry);

    // update reg A value
    self.regs.a = result;

    // update CPU flags
    self.regs.set_flag(Flags::Z, result == 0);
    self.regs.set_flag(Flags::N, false);
    self.regs.set_flag(Flags::H, (a & 0xf) + (value & 0xf) + carry > 0xf);
    self.regs.set_flag(Flags::C, (a as u16) + (value as u16) + (carry as u16) > 0xff);
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

  /// make a compare of A with byte
  fn alu_cp(&mut self, byte: u8) {
    // get current reg A value
    let old_a = self.regs.a;

    // make a subtraction
    self.alu_sub(byte, false);

    // reset the A value to the old one
    self.regs.a = old_a;
  }

  /// make a xor operation in A
  fn alu_xor(&mut self, byte: u8) {
    // make the logical exclusive or operation
    let result = self.regs.a ^ byte;

    // update reg A value
    self.regs.a = result;

    // update CPU flags
    self.regs.set_flag(Flags::Z, result == 0);
    self.regs.set_flag(Flags::N, false);
    self.regs.set_flag(Flags::H, false);
    self.regs.set_flag(Flags::C, false);
  }
}
//! Game Boy CPU emulation

use super::io::Interconnect;
use self::registers::Registers;
use std::fmt::{Debug, Formatter, Error};

mod registers;

/// CPU state
pub struct Cpu {
    /// CPU registers
    regs: Registers,
    /// Interconnect
    interconnect: Interconnect,
    /// CPU halted flag
    halted: bool,
    // interrupts are enabled?
    ime: bool,
    // is to disable interrupts
    setdi: u8,
    // is to enabled interrupts
    setei: u8
}

impl Cpu {
    /// Create a new Cpu instance and reset it
    pub fn new(inter: Interconnect) -> Cpu {
        Cpu {
            regs: Registers::new(),
            halted: false,
            interconnect: inter,
            ime: false,
            setdi: 0,
            setei: 0
        }
    }

    /// create a new cpu instance for GBC and reset it
    pub fn new_gbc(inter: Interconnect) -> Cpu {
        Cpu {
            regs: Registers::new_gbc(),
            ..Cpu::new(inter)
        }
    }

    /// fetch one byte from the interconnect using PC
    fn fetch_byte(&mut self) -> u8 {
        let byte = self.interconnect.read_byte(self.regs.pc);
        self.regs.pc += 1;
        byte
    }

    /// fetch one word from the interconnect using PC
    fn fetch_word(&mut self) -> u16 {
        let word = self.interconnect.read_word(self.regs.pc);
        self.regs.pc += 2;
        word
    }

    /// update IME (Interrupt Master Enable) if needed
    fn updateIme(&mut self) {
        self.setdi = match self.setdi {
            2 => 1,
            1 => {
                self.ime = false;
                0
            }
            _ => 0
        };

        self.setei = match self.setei {
            2 => 1,
            1 => {
                self.ime = true;
                0
            }
            _ => 0
        }
    }

    /// start the CPU
    pub fn run(&mut self) {
        loop {
            // TODO: remove this when implement the GDB
            // println!("{:?}", self);

            // process the next instruction
            let cycles = self.do_internal_cycle() * 4 as u32;

            // do the interconnect cycle
            self.interconnect.do_cycle(cycles);
        }
    }

    /// do the internal CPU cycle
    fn do_internal_cycle(&mut self) -> u32 {
        // update IME if needed
        self.updateIme();

        // check if some interrupt needs to be handled.
        // if a interrupt was handled return with the
        // number of ticks
        match self.handle_interrupt() {
            // no interrupt handled
            0 => {}

            // a interrupt was handled, return the number
            // of ticks
            t => return t,
        }

        // only process instructions if the CPU is not
        // halted
        if self.halted {
            // emulate an nop instruction
            1
        } else {
            self.process_next_instruction() as u32
        }
    }

    /// push a value to stack
    fn stack_push(&mut self, value: u16) {
        self.regs.sp -= 2;
        self.interconnect.write_word(self.regs.sp, value);
    }

    /// pop a value from stack
    fn stack_pop(&mut self) -> u16 {
        let value = self.interconnect.read_word(self.regs.sp);
        self.regs.sp += 2;
        value
    }

    /// handle the interrupts
    fn handle_interrupt(&mut self) -> u32 {
        // check if the interrupts are enabled
        if self.ime == false && self.halted == false { return 0 }

        // check if some interrupt are been fired
        let triggered = self.interconnect.inte & self.interconnect.intf;
        if triggered == 0 { return 0 }

        self.halted = false;
        if self.ime == false { return 0 }
        self.ime = false;

        // valid the interrupt
        let n = triggered.trailing_zeros();
        if n >= 5 { panic!("Invalid interrupt triggered"); }

        // clean up the interrupt
        self.interconnect.intf &= !(1 << n);

        // save the current program pointer
        let pc = self.regs.pc;
        self.stack_push(pc);

        // change the PC value to the interrupt location
        // handler
        self.regs.pc = 0x0040 | ((n as u16) << 3);

        // number of tickets
        4
    }

    fn process_next_instruction(&mut self) -> u8 {
        // fetch a byte from the PC address
        let opcode = self.fetch_byte();

        // get current regs state
        let regs = self.regs;

        match opcode {
            // NOP
            0x00 => { 1 }
            // LD (BC),nn
            0x01 => {
                let word = self.fetch_word();
                self.regs.set_bc(word);
                3
            }
            // LD (BC),A
            0x02 => {
                self.interconnect.write_byte(regs.bc(), regs.a);
                2
            }
            // INC BC
            0x03 => {
                self.regs.set_bc(regs.bc().wrapping_add(1));
                2
            }
            // INC B
            0x04 => {
                self.regs.b = self.alu_inc(regs.b);
                1
            }
            // DEC B
            0x05 => {
                self.regs.b = self.alu_dec(regs.b);
                1
            }
            // LD (nn),SP
            0x08 => {
                let word = self.fetch_word();
                self.interconnect.write_word(word, regs.sp);
                5
            }
            // LD B,n
            0x06 => {
                self.regs.b = self.fetch_byte();
                2
            }
            // ADD HL, BC
            0x09 => {
                self.alu_add16(regs.bc());
                2
            }
            // DEC BC
            0x0b => {
                self.regs.set_bc(regs.bc().wrapping_sub(1));
                2
            }
            // LD DE,nn
            0x11 => {
                let word = self.fetch_word();
                self.regs.set_de(word);
                3
            }
            // ADD HL, DE
            0x19 => {
                self.alu_add16(regs.de());
                2
            }
            // LD A, (BC)
            0x0a => {
                self.regs.a = self.interconnect.read_byte(regs.bc());
                2
            }
            // INC C
            0x0c => {
                self.regs.c = self.alu_inc(regs.c);
                1
            }
            // DEC C
            0x0d => {
                self.regs.c = self.alu_dec(regs.c);
                1
            }
            // STOP
            0x10 => {
                self.interconnect.switch_speed();
                1
            },
            // LD (DE),A
            0x12 => {
                self.interconnect.write_byte(regs.de(), regs.a);
                2
            }
            // INC DE
            0x13 => {
                self.regs.set_de(regs.de().wrapping_add(1));
                2
            }
            // INC D
            0x14 => {
                self.regs.d = self.alu_inc(regs.d);
                1
            }
            // DEC D
            0x15 => {
                self.regs.d = self.alu_dec(regs.d);
                1
            }
            // RLCA
            0x17 => {
                self.regs.a = self.alu_rl(regs.a);
                self.regs.flags.z = false;
                1
            }
            // JP (HL)
            0x18 => {
                self.regs.pc = regs.pc + (self.fetch_byte() as u16);
                1
            }
            // DEC DE
            0x1b => {
                self.regs.set_de(regs.de().wrapping_sub(1));
                2
            }
            // INC E
            0x1c => {
                self.regs.e = self.alu_inc(regs.e);
                1
            }
            // DEC E
            0x1d => {
                self.regs.e = self.alu_dec(regs.e);
                1
            }
            // LD C,n
            0x0e => {
                self.regs.c = self.fetch_byte();
                2
            }
            // LD D,n
            0x16 => {
                self.regs.d = self.fetch_byte();
                2
            }
            // LD A,(DE)
            0x1a => {
                self.regs.a = self.interconnect.read_byte(regs.de());
                2
            }
            // LD E,n
            0x1e => {
                self.regs.e = self.fetch_byte();
                2
            }
            // JR NZ,n
            0x20 => {
                if !self.regs.flags.z {
                    self.cpu_jr();
                    3
                } else {
                    self.regs.pc += 1;
                    2
                }
            }
            // LD HL,nn
            0x21 => {
                let word = self.fetch_word();
                self.regs.set_hl(word);
                3
            }
            // LD (HLI),A
            0x22 => {
                self.interconnect.write_byte(self.regs.hli(), regs.a);
                2
            }
            // INC HL
            0x23 => {
                self.regs.set_hl(regs.hl().wrapping_add(1));
                2
            }
            // INC H
            0x24 => {
                self.regs.h = self.alu_inc(regs.h);
                1
            }
            // DEC H
            0x25 => {
                self.regs.h = self.alu_dec(regs.h);
                1
            }
            // LD H,n
            0x26 => {
                self.regs.h = self.fetch_byte();
                2
            }
            // DAA
            0x27 => {
                self.alu_daa();
                1
            }
            // JR cc,n
            0x28 => {
                if self.regs.flags.z {
                    self.cpu_jr();
                    3
                } else {
                    self.regs.pc += 1;
                    2
                }
            }
            // ADD HL, HL
            0x29 => {
                self.alu_add16(regs.hl());
                2
            }
            // LDI A,(HL)
            0x2a => {
                self.regs.a = self.interconnect.read_byte(self.regs.hli());
                2
            }
            // DEC HL
            0x2b => {
                self.regs.set_hl(regs.hl().wrapping_sub(1));
                2
            }
            // INC L
            0x2c => {
                self.regs.l = self.alu_inc(regs.l);
                1
            }
            // LD L,n
            0x2e => {
                self.regs.l = self.fetch_byte();
                2
            }
            // RET
            0xc9 => {
                self.regs.pc = self.stack_pop();
                4
            }
            // JR NC,n
            0x30 => {
                if !self.regs.flags.c {
                    self.cpu_jr();
                    3
                } else {
                    self.regs.pc += 1;
                    2
                }
            }
            // LD SP,nn
            0x31 => {
                let word = self.fetch_word();
                self.regs.set_sp(word);
                3
            }
            // LD (HLD)
            0x32 => {
                self.interconnect.write_byte(self.regs.hld(), regs.a);
                2
            }
            // INC SP
            0x33 => {
                self.regs.sp = regs.sp.wrapping_add(1);
                2
            }
            // INC (HL)
            0x34 => {
                let mut value = self.interconnect.read_byte(regs.hl());
                value = self.alu_inc(value);
                self.interconnect.write_byte(regs.hl(), value);
                3
            }
            // DEC (HL)
            0x35 => {
                let mut value = self.interconnect.read_byte(regs.hl());
                value = self.alu_dec(value);
                self.interconnect.write_byte(regs.hl(), value);
                3
            }
            // LD (HL),n
            0x36 => {
                let byte = self.fetch_byte();
                self.interconnect.write_byte(regs.hl(), byte);
                3
            }
            // JR C,n
            0x38 => {
                if self.regs.flags.c {
                    self.cpu_jr();
                    3
                } else {
                    self.regs.pc += 1;
                    2
                }
            }
            // ADD HL, SP
            0x39 => {
                self.alu_add16(regs.sp);
                2
            }
            // LD A,(HLD)
            0x3a => {
                self.regs.a = self.interconnect.read_byte(self.regs.hld());
                2
            }
            // DEC SP
            0x3b => {
                self.regs.sp = regs.sp.wrapping_sub(1);
                2
            }
            // INC A
            0x3c => {
                self.regs.a = self.alu_inc(regs.a);
                1
            }
            // DEC A
            0x3d => {
                self.regs.a = self.alu_dec(regs.a);
                1
            }
            // LD A,n
            0x3e => {
                self.regs.a = self.fetch_byte();
                2
            }
            // LD B,B
            0x40 => { 1 }
            // LD B,C
            0x41 => {
                self.regs.b = regs.c;
                1
            }
            // LD B,C
            0x42 => {
                self.regs.b = regs.c;
                1
            }
            // LD B,E
            0x43 => {
                self.regs.b = regs.e;
                1
            }
            // LD B,H
            0x44 => {
                self.regs.b = regs.h;
                1
            }
            // LD B,L
            0x45 => {
                self.regs.b = regs.l;
                1
            }
            // LD B,(HL)
            0x46 => {
                self.regs.b = self.interconnect.read_byte(regs.hl());
                2
            }
            // LD B,A
            0x47 => {
                self.regs.b = regs.a;
                1
            }
            // LD C,B
            0x48 => {
                self.regs.c = regs.b;
                1
            }
            // LD C,C
            0x49 => { 1 }
            // LD C,D
            0x4a => {
                self.regs.c = regs.d;
                1
            }
            // LD C,E
            0x4b => {
                self.regs.c = regs.e;
                1
            }
            // LD C,H
            0x4c => {
                self.regs.c = regs.h;
                1
            }
            // LD C,L
            0x4d => {
                self.regs.c = regs.l;
                1
            }
            // LD C,(HL)
            0x4e => {
                self.regs.c = self.interconnect.read_byte(regs.hl());
                2
            }
            // LD C,A
            0x4f => {
                self.regs.c = regs.a;
                1
            }
            // LD D,B
            0x50 => {
                self.regs.d = regs.b;
                1
            }
            // LD D,C
            0x51 => {
                self.regs.d = regs.c;
                1
            }
            // LD D,D
            0x52 => { 1 }
            // LD D,E
            0x53 => {
                self.regs.d = regs.e;
                1
            }
            // LD D,H
            0x54 => {
                self.regs.d = regs.h;
                1
            }
            // LD D,L
            0x55 => {
                self.regs.d = regs.l;
                1
            }
            // LD D,(HL)
            0x56 => {
                self.regs.d = self.interconnect.read_byte(regs.hl());
                2
            }
            // LD D,A
            0x57 => {
                self.regs.d = regs.a;
                1
            }
            // LD E,B
            0x58 => {
                self.regs.e = regs.b;
                1
            }
            // LD E,C
            0x59 => {
                self.regs.e = regs.c;
                1
            }
            // LD E,D
            0x5a => {
                self.regs.e = regs.d;
                1
            }
            // LD E,E
            0x5b => { 1 }
            // LD E,H
            0x5c => {
                self.regs.e = regs.h;
                1
            }
            // LD E,L
            0x5d => {
                self.regs.e = regs.l;
                1
            }
            // LD E,(HL)
            0x5e => {
                self.regs.e = self.interconnect.read_byte(regs.hl());
                2
            }
            // LD E,A
            0x5f => {
                self.regs.e = regs.a;
                1
            }
            // LD H,B
            0x60 => {
                self.regs.h = regs.b;
                1
            }
            // LD H,C
            0x61 => {
                self.regs.h = regs.c;
                1
            }
            // LD H,D
            0x62 => {
                self.regs.h = regs.d;
                1
            }
            // LD H,E
            0x63 => {
                self.regs.h = regs.e;
                1
            }
            // LD H,H
            0x64 => { 1 }
            // LD H,L
            0x65 => {
                self.regs.h = regs.l;
                1
            }
            // LD H,(HL)
            0x66 => {
                self.regs.h = self.interconnect.read_byte(regs.hl());
                2
            }
            // LD H,A
            0x67 => {
                self.regs.h = regs.a;
                1
            }
            // LD L,B
            0x68 => {
                self.regs.l = regs.b;
                1
            }
            // LD L,C
            0x69 => {
                self.regs.l = regs.c;
                1
            }
            // LD L,D
            0x6a => {
                self.regs.l = regs.d;
                1
            }
            // LD L,E
            0x6b => {
                self.regs.l = regs.e;
                1
            }
            // LD L,H
            0x6c => {
                self.regs.l = regs.h;
                1
            }
            // LD L,L
            0x6d => { 1 }
            // LD L,(HL)
            0x6e => {
                self.regs.l = self.interconnect.read_byte(regs.hl());
                2
            }
            // LD L,A
            0x6f => {
                self.regs.l = regs.a;
                1
            }
            // LD (HL),B
            0x70 => {
                self.interconnect.write_byte(regs.hl(), regs.b);
                2
            }
            // LD (HL),C
            0x71 => {
                self.interconnect.write_byte(regs.hl(), regs.c);
                2
            }
            // LD (HL),D
            0x72 => {
                self.interconnect.write_byte(regs.hl(), regs.d);
                2
            }
            // LD (HL),E
            0x73 => {
                self.interconnect.write_byte(regs.hl(), regs.e);
                2
            }
            // LD (HL),H
            0x74 => {
                self.interconnect.write_byte(regs.hl(), regs.h);
                2
            }
            // LD (HL),L
            0x75 => {
                self.interconnect.write_byte(regs.hl(), regs.l);
                2
            }
            // HALT
            0x76 => {
                self.halted = true;
                1
            }
            // LD (HL),A
            0x77 => {
                self.interconnect.write_byte(regs.hl(), regs.a);
                2
            }
            // LD A,B
            0x78 => {
                self.regs.a = regs.b;
                1
            }
            // LD A,C
            0x79 => {
                self.regs.a = regs.c;
                1
            }
            // LD A,D
            0x7a => {
                self.regs.a = regs.d;
                1
            }
            // LD A,E
            0x7b => {
                self.regs.a = regs.e;
                1
            }
            // LD A,H
            0x7c => {
                self.regs.a = regs.h;
                1
            }
            // LD A,L
            0x7d => {
                self.regs.a = regs.l;
                1
            }
            // LD A,(HL)
            0x7e => {
                self.regs.a = self.interconnect.read_byte(regs.hl());
                2
            }
            // LD A,A
            0x7f => { 1 }
            // ADD A,B
            0x80 => {
                self.alu_add(regs.b, false);
                1
            }
            // ADD A,C
            0x81 => {
                self.alu_add(regs.c, false);
                1
            }
            // ADD A,D
            0x82 => {
                self.alu_add(regs.d, false);
                1
            }
            // ADD A,E
            0x83 => {
                self.alu_add(regs.e, false);
                1
            }
            // ADD A,H
            0x84 => {
                self.alu_add(regs.h, false);
                1
            }
            // ADD A,L
            0x85 => {
                self.alu_add(regs.l, false);
                1
            }
            // ADD A,(HL)
            0x86 => {
                let value = self.interconnect.read_byte(regs.hl());
                self.alu_add(value, true);
                2
            }
            // ADD A,A
            0x87 => {
                self.alu_add(regs.a, false);
                1
            }
            // ADC A,B
            0x88 => {
                self.alu_add(regs.b, true);
                1
            }
            // ADC A,C
            0x89 => {
                self.alu_add(regs.c, true);
                1
            }
            // ADC A,D
            0x8a => {
                self.alu_add(regs.d, true);
                1
            }
            // ADC A,E
            0x8b => {
                self.alu_add(regs.e, true);
                1
            }
            // ADC A,H
            0x8c => {
                self.alu_add(regs.h, true);
                1
            }
            // ADC A,L
            0x8d => {
                self.alu_add(regs.l, true);
                1
            }
            // ADC A,(HL)
            0x8e => {
                let value = self.interconnect.read_byte(regs.hl());
                self.alu_add(value, true);
                2
            }
            // ADC A,A
            0x8f => {
                self.alu_add(regs.a, true);
                1
            }
            // SUB B
            0x90 => {
                self.alu_sub(regs.b, false);
                1
            }
            // SUB C
            0x91 => {
                self.alu_sub(regs.c, false);
                1
            }
            // SUB D
            0x92 => {
                self.alu_sub(regs.d, false);
                1
            }
            // SUB E
            0x93 => {
                self.alu_sub(regs.e, false);
                1
            }
            // SUB H
            0x94 => {
                self.alu_sub(regs.h, false);
                1
            }
            // SUB L
            0x95 => {
                self.alu_sub(regs.l, false);
                1
            }
            // SUB (HL)
            0x96 => {
                let value = self.interconnect.read_byte(regs.hl());
                self.alu_sub(value, false);
                2
            }
            // SBC B
            0x98 => {
                self.alu_sub(regs.b, true);
                1
            }
            // SBC C
            0x99 => {
                self.alu_sub(regs.c, true);
                1
            }
            // SBC D
            0x9a => {
                self.alu_sub(regs.d, true);
                1
            }
            // SBC E
            0x9b => {
                self.alu_sub(regs.e, true);
                1
            }
            // SBC H
            0x9c => {
                self.alu_sub(regs.h, true);
                1
            }
            // SBC L
            0x9d => {
                self.alu_sub(regs.l, true);
                1
            }
            // SBC (HL)
            0x9e => {
                let value = self.interconnect.read_byte(regs.hl());
                self.alu_sub(value, true);
                2
            }
            // SUB A
            0x97 => {
                self.alu_sub(regs.a, false);
                1
            }
            // SBC A
            0x9f => {
                self.alu_sub(regs.a, true);
                1
            }
            // AND B
            0xa0 => {
                self.alu_and(regs.b);
                1
            }
            // AND C
            0xa1 => {
                self.alu_and(regs.c);
                1
            }
            // AND D
            0xa2 => {
                self.alu_and(regs.d);
                1
            }
            // AND E
            0xa3 => {
                self.alu_and(regs.e);
                1
            }
            // AND H
            0xa4 => {
                self.alu_and(regs.h);
                1
            }
            // AND L
            0xa5 => {
                self.alu_and(regs.l);
                1
            }
            // AND (HL)
            0xa6 => {
                let value = self.interconnect.read_byte(regs.hl());
                self.alu_and(value);
                1
            }
            // AND B
            0xa8 => {
                self.alu_and(regs.b);
                1
            }
            // AND C
            0xa9 => {
                self.alu_and(regs.c);
                1
            }
            // AND D
            0xaa => {
                self.alu_and(regs.d);
                1
            }
            // AND E
            0xab => {
                self.alu_and(regs.e);
                1
            }
            // AND H
            0xac => {
                self.alu_and(regs.h);
                1
            }
            // AND L
            0xad => {
                self.alu_and(regs.l);
                1
            }
            // AND (HL)
            0xae => {
                let value = self.interconnect.read_byte(regs.hl());
                self.alu_and(value);
                2
            }
            // AND A
            0xa7 => {
                self.alu_and(regs.a);
                1
            }
            // XOR
            0xaf => {
                self.alu_xor(regs.a);
                1
            }
            // OR B
            0xb0 => {
                self.alu_or(regs.b);
                1
            }
            // OR C
            0xb1 => {
                self.alu_or(regs.c);
                1
            }
            // OR D
            0xb2 => {
                self.alu_or(regs.d);
                1
            }
            // OR E
            0xb3 => {
                self.alu_or(regs.e);
                1
            }
            // OR H
            0xb4 => {
                self.alu_or(regs.h);
                1
            }
            // OR L
            0xb5 => {
                self.alu_or(regs.l);
                1
            }
            // OR (HL)
            0xb6 => {
                let value = self.interconnect.read_byte(regs.hl());
                self.alu_or(value);
                2
            }
            // OR A
            0xb7 => {
                self.alu_or(regs.a);
                1
            }
            // CP B
            0xb8 => {
                self.alu_cp(regs.a);
                1
            }
            // CP C
            0xb9 => {
                self.alu_cp(regs.c);
                1
            }
            // CP D
            0xba => {
                self.alu_cp(regs.d);
                1
            }
            // CP E
            0xbb => {
                self.alu_cp(regs.e);
                1
            }
            // CP H
            0xbc => {
                self.alu_cp(regs.h);
                1
            }
            // CP L
            0xbd => {
                self.alu_cp(regs.l);
                1
            }
            // CP (HL)
            0xbe => {
                let value = self.interconnect.read_byte(regs.hl());
                self.alu_cp(value);
                1
            }
            // CP A
            0xbf => {
                self.alu_cp(regs.a);
                1
            }
            // POP BC
            0xc1 => {
                let value = self.stack_pop();
                self.regs.set_bc(value);
                3
            }
            // JP nn
            0xc3 => {
                self.regs.pc = self.fetch_word();
                4
            }
            // PUSH BC
            0xc5 => {
                self.stack_push(regs.bc());
                4
            }
            // ADD A,#
            0xc6 => {
                let value = self.fetch_byte();
                self.alu_add(value, false);
                2
            }
            // CB Opcodes
            0xcb => { self.process_cb_opcodes() }
            // CALL nn
            0xcd => {
                self.stack_push(regs.pc + 2);
                let value = self.fetch_word();
                self.regs.pc = value;
                6
            }
            // ADC A,#
            0xce => {
                let value = self.fetch_byte();
                self.alu_add(value, true);
                2
            }
            // POP DE
            0xd1 => {
                let value = self.stack_pop();
                self.regs.set_de(value);
                3
            }
            // PUSH DE
            0xd5 => {
                self.stack_push(regs.de());
                3
            }
            // SUB #
            0xd6 => {
                let value = self.fetch_byte();
                self.alu_sub(value, false);
                2
            }
            // SUC #
            0xde => {
                let value = self.fetch_byte();
                self.alu_sub(value, true);
                2
            }
            // LDH (n),a
            0xe0 => {
                let byte = self.fetch_byte() as u16;
                self.interconnect.write_byte(0xff00 + byte, regs.a);
                3
            }
            // POP HL
            0xe1 => {
                let value = self.stack_pop();
                self.regs.set_hl(value);
                3
            }
            // LD (0xff00+C),A
            0xe2 => {
                self.interconnect.write_byte(0xff00 | regs.c as u16, regs.a);
                2
            }
            // PUSH hl
            0xe5 => {
                self.stack_push(regs.hl());
                3
            }
            // AND #
            0xe6 => {
                let value = self.fetch_byte();
                self.alu_and(value);
                1
            }
            // ADD SP, #
            0xe8 => {
                self.regs.sp = self.alu_add16imm(regs.sp);
                4
            }
            // LD (nn),A
            0xea => {
                let word = self.fetch_word();
                self.interconnect.write_byte(word, regs.a);
                3
            }
            // AND #
            0xee => {
                let value = self.fetch_byte();
                self.alu_and(value);
                2
            }
            // LDH A,(n)
            0xf0 => {
                let address = 0xff00 | self.fetch_byte() as u16;
                self.regs.a = self.interconnect.read_byte(address);
                3
            }
            // POP AF
            0xf1 => {
                let value = self.stack_pop();
                self.regs.set_af(value);
                3
            }
            // LD A,(0xff00+C)
            0xf2 => {
                self.regs.a = self.interconnect.read_byte(0xff00 | regs.c as u16);
                2
            }
            // DI
            0xf3 => {
                self.setdi = 2;
                1
            }
            // PUSH AF
            0xf5 => {
                self.stack_push(regs.af());
                4
            }
            // OR #
            0xf6 => {
                let value = self.fetch_byte();
                self.alu_or(value);
                2
            }
            // LDHL SP,n
            0xf8 => {
                let result = self.alu_add16imm(regs.sp);
                self.regs.set_hl(result);
                2
            }
            // LD SP,HL
            0xf9 => {
                self.regs.set_sp(regs.hl());
                2
            }
            // LD A,(nn)
            0xfa => {
                let word = self.fetch_word();
                self.regs.a = self.interconnect.read_byte(word);
                3
            }
            // EI
            0xfb => {
                self.setei = 2;
                1
            }
            // CP n
            0xfe => {
                let byte = self.fetch_byte();
                self.alu_cp(byte);
                2
            }
            _ => panic!("Unrecognized opcode: {:#x}", opcode),
        }
    }

    fn process_cb_opcodes(&mut self) -> u8 {
        // get opcode
        let opcode = self.fetch_byte();

        // save the current regs start
        let regs = self.regs;

        match opcode {
            // RL B
            0x10 => { self.regs.b = self.alu_rl(regs.b); 2  }
            // RL C
            0x11 => { self.regs.c = self.alu_rl(regs.c); 2 }
            // RL D
            0x12 => { self.regs.d = self.alu_rl(regs.d); 2 }
            // RL E
            0x13 => { self.regs.e = self.alu_rl(regs.e); 2 }
            // RL H
            0x14 => { self.regs.h = self.alu_rl(regs.h); 2 }
            // RL L
            0x15 => { self.regs.l = self.alu_rl(regs.l); 2 }
            // RL (HL)
            0x16 => {
                let value = self.interconnect.read_byte(regs.hl());
                let new_value = self.alu_rl(value);
                self.interconnect.write_byte(regs.hl(), new_value);
                4
            }
            // RL A
            0x17 => { self.regs.a = self.alu_rl(regs.a); 2 }
            // bit
            0x7c => { self.alu_bit(regs.h, 7); 2 }
            // SET 7,HL
            0xfe => {
                let address = regs.hl();
                let value = self.interconnect.read_byte(address) | (1 << 7);
                self.interconnect.write_byte(address, value);
                4
            }
            _ => panic!("Unrecognized CB opcode {:#x}", opcode),
        }
    }

    fn alu_daa(&mut self) {
        // get reg A value
        let mut a = self.regs.a;
        let mut adjust = if self.regs.flags.c { 0x60 } else { 0x00 };

        if self.regs.flags.h { adjust |= 0x06; };

        if !self.regs.flags.n {
            if a & 0x0f > 0x09 { adjust |= 0x06; };
            if a > 0x99 { adjust |= 0x60; };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_add(adjust);
        }

        // update flags
        self.regs.flags.c = adjust >= 0x60;
        self.regs.flags.h = false;
        self.regs.flags.z = a == 0;

        // update reg A value
        self.regs.a = a;
    }

    /// Update flags after some rotation operations.
    fn alu_srflagupdate(&mut self, result: u8, carry: bool) {
        self.regs.flags.h = false;
        self.regs.flags.n = false;
        self.regs.flags.c = carry;
        self.regs.flags.z = result == 0;
    }

    /// Rotate value thought Carry flag
    fn alu_rl (&mut self, value: u8) -> u8 {
        let carry = value & 0x80 == 0x80;

        // compute the new value
        let result = (value << 1) | (if self.regs.flags.c { 1 } else { 0 });

        // updates flags
        self.alu_srflagupdate(result, carry);

        result
    }

    /// test bit v1 in v2
    fn alu_bit(&mut self, v1: u8, v2: u8) {
        let result = v1 & (1 << (v2 as u32)) == 0;
        self.regs.flags.n = false;
        self.regs.flags.h = true;
        self.regs.flags.z = result;
    }

    /// decrement a value and update the CPU flags
    fn alu_dec(&mut self, value: u8) -> u8 {
        // compute the decrement
        let result = value.wrapping_sub(1);

        // update CPU flags
        self.regs.flags.z = result == 0;
        self.regs.flags.n = true;
        self.regs.flags.h = (value & 0x0f) == 0;

        // return the result
        result
    }

    /// increment a value and update the CPU flags
    fn alu_inc(&mut self, value: u8) -> u8 {
        // compute the increment
        let result = value.wrapping_add(1);

        // update CPU flags
        self.regs.flags.z = result == 0;
        self.regs.flags.n = false;
        self.regs.flags.h = (value & 0xf) + 1 > 0xf;

        // return the result
        result
    }

    fn alu_or(&mut self, value: u8) {
        // make the logical or operation
        let result = self.regs.a | value;

        // update reg A value
        self.regs.a = result;

        // update CPU flags
        self.regs.flags.z = result == 0;
        self.regs.flags.n = false;
        self.regs.flags.h = false;
        self.regs.flags.c = false;
    }

    /// make a logical and with the reg A
    fn alu_and(&mut self, value: u8) {
        // make the logical and operation
        let result = self.regs.a & value;

        // update reg A value
        self.regs.a = result;

        // update CPU flags
        self.regs.flags.z = result == 0;
        self.regs.flags.n = false;
        self.regs.flags.h = true;
        self.regs.flags.c = false;
    }

    /// add immediate to sp and update CPU flags
    fn alu_add16imm(&mut self, sp: u16) -> u16 {
        // cast byte to the correct type
        let byte = self.fetch_byte() as i8 as i16 as u16;

        // update the CPU flags
        self.regs.flags.z = false;
        self.regs.flags.n = false;
        self.regs.flags.h = byte & 0x000f > 0x000f;
        self.regs.flags.c = byte & 0x00ff > 0x00ff;

        // compute the addiction and return it
        sp.wrapping_add(byte)
    }

    /// add a value to reg HL
    fn alu_add16(&mut self, value: u16) {
        // compute the addiction
        let result = self.regs.hl().wrapping_add(value);

        // update reg HL reg
        self.regs.set_hl(result);

        // update CPU flags
        self.regs.flags.n = false;
        self.regs.flags.h = (result & 0xfff) + (value & 0xfff) > 0xfff;
        self.regs.flags.c = result > 0xffff - value;
    }

    /// add a value to reg A
    fn alu_add(&mut self, value: u8, usec: bool) {
        // get carry (if is to use)
        let carry = if usec && self.regs.flags.c { 1 } else { 0 };

        // get reg a value
        let a = self.regs.a;

        // compute the new value
        let result = a.wrapping_add(value).wrapping_add(carry);

        // update reg A value
        self.regs.a = result;

        // update CPU flags
        self.regs.flags.z = result == 0;
        self.regs.flags.n = false;
        self.regs.flags.h = (a & 0xf) + (value & 0xf) + carry > 0xf;
        self.regs.flags.c = (a as u16) + (value as u16) + (carry as u16) > 0xff;
    }

    /// perform a subtraction
    fn alu_sub(&mut self, byte: u8, usec: bool) {
        // get carry
        let c = if usec && self.regs.flags.c { 1 } else { 0 };

        // get reg A value
        let a = self.regs.a;

        // compute the result
        let result = a.wrapping_sub(byte).wrapping_sub(c);

        // update the value of reg A
        self.regs.a = result;

        // update CPU flags
        self.regs.flags.z = result == 0;
        self.regs.flags.n = true;
        self.regs.flags.h = (a & 0x0f) < (byte & 0x0f) + c;
        self.regs.flags.c = (a as u16) < (byte as u16) + (c as u16);
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
        self.regs.flags.z = result == 0;
        self.regs.flags.n = false;
        self.regs.flags.h = false;
        self.regs.flags.c = false;
    }

    /// process the relative jump
    fn cpu_jr(&mut self) {
        // get the offset
        let off = self.fetch_byte() as i8;

        // get program counter
        let mut pc = self.regs.pc as i16;

        // compute the new PC address
        pc = pc.wrapping_add(off as i16);

        // save the new PC value
        self.regs.pc = pc as u16;
    }
}

/// CPU Debugger
impl Debug for Cpu {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        // --- Print Registers
        writeln!(f, "Registers");

        // Program Counter
        writeln!(f, "  pc: 0x{:04x} [{:02X} {:02X} {:02x} ...]",
                 self.regs.pc,
                 self.interconnect.read_byte(self.regs.pc),
                 self.interconnect.read_byte(self.regs.pc.wrapping_add(1)),
                 self.interconnect.read_byte(self.regs.pc.wrapping_add(2)));

        // Stack pointer
        writeln!(f, "  sp: 0x{:04x} [{:02X} {:02X} {:02x} ...]",
                 self.regs.sp,
                 self.interconnect.read_byte(self.regs.sp),
                 self.interconnect.read_byte(self.regs.sp.wrapping_add(1)),
                 self.interconnect.read_byte(self.regs.sp.wrapping_add(2)));

        // registers
        writeln!(f, "  af: 0x{:04x}    a: {:3}    f: {:3}",
                 self.regs.af(),
                 self.regs.a,
                 self.regs.f());

        writeln!(f, "  bc: 0x{:04x}    b: {:3}    c: {:3}",
                 self.regs.bc(),
                 self.regs.b,
                 self.regs.c);

        writeln!(f, "  de: 0x{:04x}    d: {:3}    e: {:3}",
                 self.regs.de(),
                 self.regs.d,
                 self.regs.e);

        writeln!(f, "  hl: 0x{:04x}    h: {:3}    l: {:3}    \
            [hl]: [{:02X} {:02X} ...]",
                 self.regs.hl(),
                 self.regs.h,
                 self.regs.l,
                 self.interconnect.read_byte(self.regs.hl()),
                 self.interconnect.read_byte(self.regs.hl() + 1));

        // --- Flags
        writeln!(f, "Flags:");

        writeln!(f, "  z: {}  n: {}  h: {}  c: {}",
                 self.regs.flags.z as u8,
                 self.regs.flags.n as u8,
                 self.regs.flags.h as u8,
                 self.regs.flags.c as u8);

        // CPU State
        writeln!(f, "  ime: {}   halted: {}", self.ime, self.halted);

        // finish print
        Ok(())
    }
}

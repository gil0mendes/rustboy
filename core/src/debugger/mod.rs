use std::io::*;

use crate::machine::Machine;

use self::command::Command;

mod command;

pub struct Debugger {
  machine: Machine,
  last_command: Option<Command>,
}

impl Debugger {
  /// Create a new Debugger instance.
  pub fn new(machine: Machine) -> Self {
    Self {
      machine,
      last_command: None,
    }
  }

  /// Run the debugger
  pub fn run(&mut self) {
    loop {
      print!("rustboy> ");
      stdout().flush().unwrap();


      let command = match (read_stdio().parse(), self.last_command) {
        (Ok(Command::Repeat), Some(c)) => Ok(c),
        (Ok(Command::Repeat), None) => Err("No last command".into()),
        (Ok(c), _) => Ok(c),
        (Err(e), _) => Err(e),
      };
      
      match command {
        Ok(Command::Step(count)) => self.step(count),
        Ok(Command::Exit) => break,
        Ok(Command::Repeat) => unreachable!(),
        Err(ref e) => println!("{}", e),
      }

      self.last_command = command.ok();
    }
  }

  fn step(&mut self, count: usize) {
    for _ in 0..count {
      print_cpu_state(&self.machine);
      self.machine.emulate();
    }
  }
}

/// Read from the keyboard
fn read_stdio() -> String {
  let mut input = String::new();
  stdin().read_line(&mut input).unwrap();
  input.trim().into()
}

/// Print the current CPU state
fn print_cpu_state(machine: &Machine) {
  let cpu = &machine.cpu;
  let interconnect = &machine.interconnect;

  // Print Registers
  println!("Registers");

  // Program Counter
  println!("  pc: 0x{:04x} [{:02X} {:02X} {:02x} ...]",
            cpu.regs.pc,
            interconnect.read_byte(cpu.regs.pc),
            interconnect.read_byte(cpu.regs.pc.wrapping_add(1)),
            interconnect.read_byte(cpu.regs.pc.wrapping_add(2)));

  // Stack pointer
  println!("  sp: 0x{:04x} [{:02X} {:02X} {:02x} ...]",
            cpu.regs.sp,
            interconnect.read_byte(cpu.regs.sp),
            interconnect.read_byte(cpu.regs.sp.wrapping_add(1)),
            interconnect.read_byte(cpu.regs.sp.wrapping_add(2)));

  // registers
  println!(
      "  af: 0x{:04x}    a: {:3}    f: {:3}",
      cpu.regs.af(),
      cpu.regs.a,
      cpu.regs.f()
  );

  println!(
      "  bc: 0x{:04x}    b: {:3}    c: {:3}",
      cpu.regs.bc(),
      cpu.regs.b,
      cpu.regs.c
  );

  println!(
      "  de: 0x{:04x}    d: {:3}    e: {:3}",
      cpu.regs.de(),
      cpu.regs.d,
      cpu.regs.e
  );

  println!(
      "  hl: 0x{:04x}    h: {:3}    l: {:3} \
      [hl]: [{:02X} {:02X} ...]",
      cpu.regs.hl(),
      cpu.regs.h,
      cpu.regs.l,
      interconnect.read_byte(cpu.regs.hl()),
      interconnect.read_byte(cpu.regs.hl() + 1)
  );

  // Flags
  println!("Flags:");

  println!(
      "  z: {}  n: {}  h: {}  c: {}",
      cpu.regs.flags.z as u8,
      cpu.regs.flags.n as u8,
      cpu.regs.flags.h as u8,
      cpu.regs.flags.c as u8
  );

  // CPU State
  println!("  ime: {}   halted: {}", cpu.ime, cpu.halted);
}

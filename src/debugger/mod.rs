use std::io::*;

use machine::Machine;

use self::command::Command;

mod command;

pub struct Debugger {
  machine: Machine
}

impl Debugger {
  /// Create a new Debugger instance.
  pub fn new(machine: Machine) -> Self {
    Self {
      machine
    }
  }

  /// Run the debugger
  pub fn run(&mut self) {
    loop {
      print!("rustboy> ");
      stdout().flush().unwrap();

      let input = read_stdio();
      let command = input.parse();

      match command {
        Ok(Command::Step) => self.step(),
        Ok(Command::Exit) => break,
        _ => println!("Invalid command")
      }
    }
  }

  fn step(&mut self) {
    print_cpu_state(&self.machine);
    self.machine.emulate();
  }
}

/// Read from the keyboard
fn read_stdio() -> String {
  let mut input = String::new();
  stdin().read_line(&mut input).unwrap();
  input.trim().into()
}

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

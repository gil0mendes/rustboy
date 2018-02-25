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
        Ok(Command::Step) => self.machine.emulate(),
        _ => println!("Invalid command")
      }
    }
  }
}

/// Read from the keyboard
fn read_stdio() -> String {
  let mut input = String::new();
  stdin().read_line(&mut input).unwrap();
  input.trim().into()
}

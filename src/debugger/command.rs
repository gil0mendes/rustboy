use std::borrow::Cow;
use std::str::{self, FromStr};

#[derive(Debug, Clone, Copy)]
pub enum Command {
  Step,
  Exit,
  Repeat,
}

impl FromStr for Command {
  type Err = Cow<'static, str>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
        "" => Ok(Command::Repeat),
        "step" | "s" => Ok(Command::Step),
        "exit" | "quit" | "e" | "q" => Ok(Command::Exit),
        _ => Err("Unable to parse command.".into()),
    }
  }
}

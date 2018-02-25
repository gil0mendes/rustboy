use std::str::FromStr;

pub enum Command {
  Step
}

impl FromStr for Command {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
        "step" => Ok(Command::Step),
        _ => Err(())
    }
  }
}

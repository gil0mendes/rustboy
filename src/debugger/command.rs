use std::borrow::Cow;
use std::str::{self, FromStr};

use nom::{IResult, eol};

#[derive(Debug, Clone, Copy)]
pub enum Command {
  Step,
  Exit,
  Repeat,
}

impl FromStr for Command {
  type Err = Cow<'static, str>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match command(s.as_bytes()) {
      IResult::Done(_, c) => Ok(c),
      err => Err(format!("Unable to parse command: {:?}", err).into()),
    }
  }
}

named!(
  command<Command>,
    do_parse!(
      c: alt!(
        step |
        exit |
        repeat) >>
      (c)
    )
);

named!(
  step<Command>,
  map!(
    alt_complete!(tag!("step") | tag!("s")),
    |_| Command::Step
  )
);

named!(
  exit<Command>,
  map!(
    alt_complete!(tag!("exit") | tag!("quit") | tag!("e") | tag!("q")),
    |_| Command::Exit
  )
);

named!(
  repeat<Command>,
  value!(Command::Repeat)
);

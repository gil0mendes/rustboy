use std::borrow::Cow;
use std::str::{self, FromStr};

use nom::{
  IResult, 
  space,
  digit,
};

#[derive(Debug, Clone, Copy)]
pub enum Command {
  Step(usize),
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
      c: alt_complete!(
        step |
        exit |
        repeat) >>
      eof!() >>
      (c)
    )
);

named!(
  step<Command>,
  do_parse!(
    alt_complete!(tag!("step") | tag!("s")) >>
    steps: opt!(preceded!(space, parse_usize)) >>
    (Command::Step(steps.unwrap_or(1)))
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

named!(
  parse_usize<usize>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

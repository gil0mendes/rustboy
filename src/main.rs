use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use cpu::Cpu;

mod cpu;

fn main() {
  // get first command line argument (rom path)
  let rom_name = env::args().nth(1).unwrap();

  // get rom buffer
  let rom_buf = read_rom(rom_name);

  // create a new Cpu
  let mut cpu = Cpu::new(rom_buf);

  // print Cpu cur state
  println!("{:?}", cpu);
}

/// Read ROM as a 8-bit integer vector
///
/// # Arguments
/// * `path` - a path for the ROM to load
fn read_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
  // read file
  let mut file = File::open(path).unwrap();

  // create a new vector
  let mut file_buffer = Vec::new();

  // get file buffer
  file.read_to_end(&mut file_buffer).unwrap();

  // return buffer
  file_buffer
}

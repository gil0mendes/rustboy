use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use cpu::Cpu;

mod cpu;

fn main() {
  // get first command line argument
  let rom_name = env::args().nth(1).unwrap();

  let rom_buf = read_bin(rom_name);

  let mut cpu = Cpu::new(rom_buf);

  println!("{:?}", cpu);
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
  // read file
  let mut file = File::open(path).unwrap();

  // get buffer
  let mut file_buffer = Vec::new();
  file.read_to_end(&mut file_buffer).unwrap();

  file_buffer
}

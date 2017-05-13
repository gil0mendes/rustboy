extern crate cpal;
extern crate piston;
extern crate piston_window;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use cpu::Cpu;
use gpu::Gpu;
use io::Interconnect;
use cartridge::Cartridge;

mod io;
mod cpu;
mod gpu;
mod sound;
mod cartridge;

fn main() {
    // get first command line argument (rom path)
    let rom_name = env::args().nth(1).unwrap();

    // get rom buffer
    let rom_buf = read_rom(rom_name);

    // create cartridge
    let mut cartridge = Cartridge::new(rom_buf);

    // start window

    // create GPU
    let mut gpu = Gpu::new();

    // create a new Interconnect instance
    let mut interconnect = Interconnect::new(cartridge, gpu);

    // create a new Cpu instance
    let mut cpu = Cpu::new(interconnect);

    // start CPU
    cpu.run();
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

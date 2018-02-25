#[macro_use]
extern crate bitflags;
extern crate cpal;
#[macro_use]
extern crate error_chain;
extern crate sdl2;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate log;
extern crate glium_sdl2;
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate nalgebra;
#[macro_use]
extern crate nom;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use cartridge::Cartridge;
use frontend::Controller;
use machine::Machine;

mod io;
mod cpu;
mod gpu;
mod frontend;
mod sound;
mod cartridge;
mod machine;
mod debugger;

fn main() {
    // get first command line argument (rom path)
    let rom_name = env::args().nth(1).unwrap();

    // Get rom buffer and create a new cartridge
    let rom_buf = read_rom(rom_name);
    let cartridge = Cartridge::new(rom_buf);

    // Create a new machine
    let machine = Machine::new(cartridge);

    let mut debugger = debugger::Debugger::new(machine);
    debugger.run();

    // Start the controller
    // match Controller::new(machine) {
    //     Ok(controller) => controller.main(),
    //     Err(e) => println!("{:?}", e)
    // };
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

#[macro_use]
extern crate bitflags;
extern crate cpal;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate nalgebra;
#[macro_use]
extern crate nom;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use cartridge::Cartridge;
pub use machine::Machine;
pub use config::Config;
pub use gpu::types::{ScreenBuffer, Color};

mod io;
mod cpu;
mod gpu;
mod sound;
mod cartridge;
mod machine;
mod debugger;
mod types;
mod config;

pub const SCREEN_X: usize = 160;
pub const SCREEN_Y: usize = 144;

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

pub fn create_emulator(rom_name: &str) -> Machine {
    // Get rom buffer and create a new cartridge
    let rom_buf = read_rom(rom_name);
    let cartridge = Cartridge::new(rom_buf);

    // Create a new machine
    Machine::new(cartridge)
//
//    if matches.is_present("debug") {
//        let mut debugger = debugger::Debugger::new(machine);
//        debugger.run();
//    } else {
//        loop {
//            machine.emulate();
//        }
    // Start the controller
//        match Controller::new(machine) {
//            Ok(controller) => controller.main(),
//            Err(e) => println!("{:?}", e)
//        };
//    }
}

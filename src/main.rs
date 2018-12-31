#[macro_use]
extern crate bitflags;
extern crate cpal;
extern crate clap;
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

use std::fs::File;
use std::io::Read;
use std::path::Path;

use clap::{
    Arg,
    ArgMatches,
    App,
};

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
mod types;

/// Build the RustBoy's command line.
fn build_command_line() -> ArgMatches<'static> {
    App::new("RustBoy")
        .version("1.0")
        .author("Gil Mendes <gil00mendes@gmail.com>")
        .about("GameBoy Emulator")
        .arg(Arg::with_name("ROM")
            .help("ROM to be used")
            .required(true)
            .index(1))
        .arg(Arg::with_name("debug")
            .short("d")
            .help("Use debug mode"))
        .get_matches()
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

fn main() {
    // Build command line and get the matched arguments
    let matches = build_command_line();

    // get first command line argument (rom path)
    let rom_name = matches.value_of("ROM").unwrap();

    // Get rom buffer and create a new cartridge
    let rom_buf = read_rom(rom_name);
    let cartridge = Cartridge::new(rom_buf);

    // Create a new machine
    let mut machine = Machine::new(cartridge);

    if matches.is_present("debug") {
        let mut debugger = debugger::Debugger::new(machine);
        debugger.run();
    } else {
        loop {
            machine.emulate();
        }
        // Start the controller
        // match Controller::new(machine) {
        //     Ok(controller) => controller.main(),
        //     Err(e) => println!("{:?}", e)
        // };
    }
}

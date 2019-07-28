extern crate clap;
extern crate sdl2;
extern crate rustboy;

use clap::{
    Arg,
    ArgMatches,
    App,
};
use rustboy::Config;
use crate::controller::Controller;
use std::{thread, time};

mod controller;
mod renderer;
mod sdl;

const FRAME_TARGET: time::Duration = time::Duration::from_millis(14);

/// Build the RustBoy's command line.
fn build_command_line() -> ArgMatches<'static> {
    App::new("RustBoy")
        .version("1.0")
        .author("Gil Mendes <gil00mendes@gmail.com>")
        .about("A GameBoy Emulator written in Rust.")
        .arg(Arg::with_name("ROM")
            .help("ROM to be used")
            .required(true)
            .index(1))
        .arg(Arg::with_name("debug")
            .short("d")
            .help("Use debug mode"))
        .get_matches()
}

fn main() {
    // Build command line and get the matched arguments and
    // get a config instance from that args.
    let matches = build_command_line();
    let config = Config::from_clap(matches);

    let mut emulator = rustboy::create_emulator(&config.rom_name);

    let mut controller = if !config.is_headless {
        Some(Controller::new(
            rustboy::SCREEN_X as u32,
            rustboy::SCREEN_Y as u32,
        ))
    } else {
        None
    };

    let normal_speed = true;

    loop {
        let now = time::SystemTime::now();

        if let Some(ref mut c) = controller {
            emulator.emulate();
            c.update_controller();
            c.refresh(&mut emulator);

            if normal_speed {
                let elapsed = now.elapsed().unwrap();
                if elapsed < FRAME_TARGET {
                    thread::sleep(FRAME_TARGET - elapsed);
                }
            }
        }
    }
}
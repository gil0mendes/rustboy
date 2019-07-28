//! Controller to the front end window.

use rustboy::Machine;
use crate::renderer::Renderer;
use crate::sdl::Context;
use super::sdl::display::Display;

#[derive(Debug)]
pub enum Event {
    Quit,
    Break,
    Continue,
}

/// Structure that controls all the front-end interactions.
pub struct Controller {
    /// Media system context
    context: Context,
    /// Display instance
    display: Display,
}

impl Controller {
    /// Create a new Controller instance
    pub fn new(x: u32, y: u32) -> Self {
        let context = Context::new();
        let display = context.new_display(x, y);

        Self {
            context,
            display,
        }
    }

    pub fn refresh(&mut self, emulator: &mut Machine) {
        {
            let pixels = emulator.screen_buffer();
            self.display.refresh(pixels);
        }

        // TODO: add support for sound
    }

    pub fn update_controller(&mut self) {
        self.context.update_controller();
    }
}
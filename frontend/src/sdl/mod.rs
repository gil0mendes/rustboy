use sdl2::Sdl;
use self::controller::Controller;

pub mod display;
mod controller;

pub struct Context {
    context: Sdl,
    controller: Controller,
}

impl Context {
    /// Build a new context instance
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let controller = Controller::new();

        Self {
            context,
            controller
        }
    }

    /// Build a new display
    pub fn new_display(&self, width: u32, height: u32) -> display::Display {
        display::Display::new(&self.context, width, height)
    }

    pub fn update_controller(&mut self) {
        self.controller.update(&self.context);
    }
}
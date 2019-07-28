use sdl2::Sdl;

pub struct Controller {}

impl Controller {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, sdl: &Sdl) {
        let mut event_pump = sdl.event_pump().unwrap();

        for e in event_pump.poll_iter() {
            match e {
                _ => ()
            }
        }
    }
}
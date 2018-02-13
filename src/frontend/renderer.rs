use glium;
use glium::backend::Facade;

pub struct Renderer {
}

impl Renderer {
    pub fn new<F: Facade>(display: &F) -> Result<Self, String> {
        Ok(Self {
        })
    }
}
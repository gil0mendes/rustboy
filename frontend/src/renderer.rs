//! Describe the render trait that all renders must implement.
use rustboy::ScreenBuffer;

pub trait Renderer {
    fn refresh(&mut self, pixels: &ScreenBuffer);
}
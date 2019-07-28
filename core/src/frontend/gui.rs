use imgui::{Ui};
use imgui_glium_renderer::RendererError;

use super::ControllerError;

impl From<RendererError> for ControllerError {
    fn from(e: RendererError) -> ControllerError {
        ControllerError::Renderer(format!("{}", e))
    }
}

pub trait Screen {
    fn render(&mut self, ui: &Ui);
}

pub struct InGameScreen {
}

impl InGameScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for InGameScreen {
    fn render(&mut self, ui: &Ui) {
    }
}
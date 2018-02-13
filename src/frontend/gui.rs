use imgui_glium_renderer::RendererError;

use super::ControllerError;

impl From<RendererError> for ControllerError {
    fn from(e: RendererError) -> ControllerError {
        ControllerError::Renderer(format!("{}", e))
    }
}
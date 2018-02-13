use glium::{Api, Surface, SwapBuffersError, Version};
use glium_sdl2::{Display, DisplayBuild, GliumSdl2Error};
use sdl2;
use sdl2::{Sdl, VideoSubsystem};
use imgui::ImGui;
use imgui_glium_renderer;

use self::renderer::Renderer;

mod renderer;
mod gui;

#[derive(Clone, Debug)]
pub enum ControllerError {
    Sdl(String),
    Renderer(String),
    Other(String)
}

/// Type to be used when returning a result
pub type ControllerResult<T> = Result<T, ControllerError>;

impl From<sdl2::IntegerOrSdlError> for ControllerError {
    fn from(e: sdl2::IntegerOrSdlError) -> ControllerError {
        ControllerError::Sdl(format!("{:?}", e))
    }
}

impl From<GliumSdl2Error> for ControllerError {
    fn from(e: GliumSdl2Error) -> ControllerError {
        ControllerError::Renderer(format!("{:?}", e))
    }
}

impl From<SwapBuffersError> for ControllerError {
    fn from(e: SwapBuffersError) -> ControllerError {
        ControllerError::Renderer(format!("{:?}", e))
    }
}

impl From<String> for ControllerError {
    fn from(e: String) -> ControllerError {
        ControllerError::Other(e)
    }
}

pub struct Controller {
    sdl: Sdl,
    sdl_video: VideoSubsystem,
    display: Display,
    renderer: Renderer,
    gui_renderer: imgui_glium_renderer::Renderer,
}

impl Controller {
    pub fn new(x: u32, y: u32) -> ControllerResult<Self> {
        let sdl = sdl2::init()?;
        let sdl_video = sdl.video()?;

        // configure_gl_attr(&mut sdl_video.gl_attr());

        let display = sdl_video.window("RustBoy", 640, 576)
            .opengl()
            .position_centered()
            .build_glium()?;

        let renderer = Renderer::new(&display)?;

        let mut imgui = ImGui::init();
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);
        let gui_renderer = imgui_glium_renderer::Renderer::init(&mut imgui, &display)?;

        Ok(Controller {
            sdl,
            sdl_video,
            display,
            renderer,
            gui_renderer
        })
    }
}
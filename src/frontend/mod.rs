use glium::{Api, Surface, SwapBuffersError, Version};
use glium_sdl2::{Display, DisplayBuild, GliumSdl2Error};
use sdl2;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::video::gl_attr::GLAttr;
use imgui::ImGui;
use imgui_glium_renderer;

use cartridge::Cartridge;
use self::renderer::Renderer;
use machine::Machine;
use self::gui::Screen;

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
    cartridge: Cartridge,
    imgui: ImGui
}

impl Controller {
    pub fn new(cartridge: Cartridge) -> ControllerResult<Self> {
        let sdl = sdl2::init()?;
        let sdl_video = sdl.video()?;

        configure_gl_attr(&mut sdl_video.gl_attr());

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
            gui_renderer,
            cartridge,
            imgui
        })
    }

    pub fn main(mut self) {
        let mut screen = gui::InGameScreen::new();
        let mut machine = Machine::new(self.cartridge);

        'main: loop {
            // Draw the screen
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);

            // Draw the window frame
            let (width, height) = target.get_dimensions();
            let ui = self.imgui.frame((width, height), (width, height), 0.1);

            // process the next instruction
            machine.emulate();

            // TODO: only apply updates when VRAM changes
            self.renderer.update_pixels(machine.screen_buffer());

            // Draw GPU buffer on window
            self.renderer.draw(&mut target);

            screen.render(&ui);
            target.finish();
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn configure_gl_attr(_: &mut GLAttr) { }

#[cfg(target_os = "macos")]
fn configure_gl_attr(gl_attr: &mut GLAttr) {
    use sdl2::video::GLProfile;
    gl_attr.set_context_major_version(3);
    gl_attr.set_context_minor_version(2);
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_flags().forward_compatible().set();
}

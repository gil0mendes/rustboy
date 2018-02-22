use glium;
use glium::backend::Facade;
use glium::texture::pixel_buffer::PixelBuffer;
use glium::texture::{ClientFormat, MipmapsOption, UncompressedFloatFormat, TextureCreationError, PixelValue};
use glium::texture::texture2d::Texture2d;

use super::{ControllerError, ControllerResult};

use gpu::types;

const TEXTURE_WIDTH: u32 = 256;
const TEXTURE_HEIGHT: u32 = 256;
const TEX_OFFSET_X: f32 = types::SCREEN_WIDTH as f32 / TEXTURE_WIDTH as f32;
const TEX_OFFSET_Y: f32 = types::SCREEN_HEIGHT as f32 / TEXTURE_HEIGHT as f32;

unsafe impl PixelValue for types::Color {
    fn get_format() -> ClientFormat {
        ClientFormat::U8
    }
}

fn upload_pixels(texture: &mut Texture2d, pixel_buffer: &PixelBuffer<types::Color>) {
    texture.main_level().raw_upload_from_pixel_buffer(
        pixel_buffer.as_slice(), 0..types::SCREEN_WIDTH as u32, 0..types::SCREEN_HEIGHT as u32, 0..1,
    );
}

pub struct Renderer {
    pixel_buffer: PixelBuffer<types::Color>,
    texture: Texture2d
}

impl From<TextureCreationError> for ControllerError {
    fn from(e: TextureCreationError) -> Self {
        ControllerError::Renderer(format!("{:?}", e))
    }
}

impl Renderer {
    pub fn new<F: Facade>(display: &F) -> ControllerResult<Self> {
        let pixel_buffer = PixelBuffer::new_empty(display, types::SCREEN_WIDTH * types::SCREEN_HEIGHT);
        pixel_buffer.write(&vec![types::Color::Off; pixel_buffer.get_size()]);

        let mut texture = Texture2d::empty_with_format(display,
                                                       UncompressedFloatFormat::U8,
                                                       MipmapsOption::NoMipmap,
                                                       TEXTURE_WIDTH, TEXTURE_HEIGHT)?;

        upload_pixels(&mut texture, &pixel_buffer);

        Ok(Self {
            pixel_buffer,
            texture
        })
    }

    /// Update the texture with the new GPU frame buffer
    pub fn update_pixels(&mut self, pixels: &types::ScreenBuffer) {
        self.pixel_buffer.write(pixels);
        upload_pixels(&mut self.texture, &self.pixel_buffer);
    }
}
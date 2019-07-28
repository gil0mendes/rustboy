use glium;
use glium::{DrawError, DrawParameters, IndexBuffer, Program, VertexBuffer, Surface};
use glium::backend::Facade;
use glium::index::PrimitiveType;
use glium::program::ProgramChooserCreationError;
use glium::texture::pixel_buffer::PixelBuffer;
use glium::texture::{ClientFormat, MipmapsOption, UncompressedFloatFormat, TextureCreationError, PixelValue};
use glium::texture::texture2d::Texture2d;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use nalgebra::{Matrix4, Vector4};

use super::{ControllerError, ControllerResult};

use gpu::types;

const TEXTURE_WIDTH: u32 = 256;
const TEXTURE_HEIGHT: u32 = 256;
const TEX_OFFSET_X: f32 = types::SCREEN_WIDTH as f32 / TEXTURE_WIDTH as f32;
const TEX_OFFSET_Y: f32 = types::SCREEN_HEIGHT as f32 / TEXTURE_HEIGHT as f32;
const ASPECT_RATIO: f32 = types::SCREEN_WIDTH as f32 / types::SCREEN_HEIGHT as f32;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

unsafe impl PixelValue for types::Color {
    fn get_format() -> ClientFormat {
        ClientFormat::U8
    }
}

impl From<DrawError> for ControllerError {
    fn from(e: DrawError) -> ControllerError {
        ControllerError::Renderer(format!("{:?}", e))
    }
}

impl From<TextureCreationError> for ControllerError {
    fn from(e: TextureCreationError) -> Self {
        ControllerError::Renderer(format!("{:?}", e))
    }
}

impl From<glium::vertex::BufferCreationError> for ControllerError {
    fn from(e: glium::vertex::BufferCreationError) -> ControllerError {
        ControllerError::Renderer(format!("{:?}", e))
    }
}

impl From<glium::index::BufferCreationError> for ControllerError {
    fn from(e: glium::index::BufferCreationError) -> ControllerError {
        ControllerError::Renderer(format!("{:?}", e))
    }
}

impl From<ProgramChooserCreationError> for ControllerError {
    fn from(e: ProgramChooserCreationError) -> ControllerError {
        ControllerError::Renderer(format!("{:?}", e))
    }
}

fn upload_pixels(texture: &mut Texture2d, pixel_buffer: &PixelBuffer<types::Color>) {
    texture.main_level().raw_upload_from_pixel_buffer(
        pixel_buffer.as_slice(), 0..types::SCREEN_WIDTH as u32, 0..types::SCREEN_HEIGHT as u32, 0..1,
    );
}

fn aspect_ratio_correction(width: u32, height: u32) -> (f32, f32) {
    let fb_aspect_ratio = width as f32 / height as f32;
    let scale = ASPECT_RATIO / fb_aspect_ratio;
    if fb_aspect_ratio >= ASPECT_RATIO { (scale, 1.0) } else { (1.0, 1.0 / scale) }
}

pub struct Renderer {
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    pixel_buffer: PixelBuffer<types::Color>,
    texture: Texture2d,
    program: Program,
    matrix: Matrix4<f32>,
    palette: Matrix4<f32>,
}

impl Renderer {
    pub fn new<F: Facade>(display: &F) -> ControllerResult<Self> {
        let vertexes = [
            Vertex { position: [-1.0, -1.0], tex_coords: [0.0, TEX_OFFSET_Y] },
            Vertex { position: [-1.0, 1.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [1.0, -1.0], tex_coords: [TEX_OFFSET_X, 0.0] },
            Vertex { position: [1.0, 1.0], tex_coords: [TEX_OFFSET_X, TEX_OFFSET_Y] },
        ];

        let vertex_buffer = VertexBuffer::immutable(display, &vertexes)?;

        let index_buffer = IndexBuffer::immutable(
            display, PrimitiveType::TriangleStrip, &[1u16, 2, 0, 3],
        )?;

        let program = program!(
            display,
            140 => {
                vertex: include_str!("shader/vert_140.glsl"),
                fragment: include_str!("shader/frag_140.glsl"),
                outputs_srgb: true
            },
            110 => {
                vertex: include_str!("shader/vert_110.glsl"),
                fragment: include_str!("shader/frag_110.glsl"),
                outputs_srgb: true
            }
        )?;

        let pixel_buffer = PixelBuffer::new_empty(display, types::SCREEN_WIDTH * types::SCREEN_HEIGHT);
        pixel_buffer.write(&vec![types::Color::Off; pixel_buffer.get_size()]);

        let mut texture = Texture2d::empty_with_format(display,
                                                       UncompressedFloatFormat::U8,
                                                       MipmapsOption::NoMipmap,
                                                       TEXTURE_WIDTH, TEXTURE_HEIGHT)?;

        upload_pixels(&mut texture, &pixel_buffer);

        let (width, height) = display.get_context().get_framebuffer_dimensions();
        let (x_scale, y_scale) = aspect_ratio_correction(width, height);
        let matrix = Matrix4::from_diagonal(&Vector4::new(x_scale, y_scale, 1.0, 1.0));

        let palette = Matrix4::new(255.0, 181.0, 107.0, 33.0,
                                   247.0, 174.0, 105.0, 32.0,
                                   123.0, 74.0, 49.0, 16.0,
                                   1.0, 1.0, 1.0, 1.0) / 255.0;

        Ok(Self {
            pixel_buffer,
            texture,
            vertex_buffer,
            index_buffer,
            program,
            matrix,
            palette,
        })
    }

    /// Update the texture with the new GPU frame buffer
    pub fn update_pixels(&mut self, pixels: &types::ScreenBuffer) {
        self.pixel_buffer.write(pixels);
        upload_pixels(&mut self.texture, &self.pixel_buffer);
    }

    pub fn draw<S: Surface>(&self, frame: &mut S) -> ControllerResult<()> {
        let matrix: &[[f32; 4]; 4] = self.matrix.as_ref();
        let palette: &[[f32; 4]; 4] = self.palette.as_ref();
        let uniforms = uniform! {
            matrix: matrix.clone(),
            palette: palette.clone(),
            tex: self.texture.sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Nearest)
        };

        let params = DrawParameters {
            ..Default::default()
        };

        frame.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniforms, &params)?;

        Ok(())
    }
}

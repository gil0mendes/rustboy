use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::Canvas;
use crate::renderer::Renderer;
use sdl2::pixels::Color as SColor;
use sdl2::rect::Point;
use rustboy::{ScreenBuffer, Color};

pub struct Display {
    canvas: Canvas<Window>
}

impl Display {
    /// Build a new Display
    pub fn new(sdl2: &Sdl, width: u32, height: u32) -> Self {
        // Build window
        let video_subsystem = sdl2.video().unwrap();
        let window = video_subsystem.window("rustboy", width, height)
            .position_centered()
            .build()
            .unwrap();

        // Create a canvas that will be used to draw in the Window
        let canvas = window
            .into_canvas()
            .software()
            .build()
            .unwrap();

        Self {
            canvas
        }
    }
}

impl Renderer for Display {
    fn refresh(&mut self, pixels: &ScreenBuffer) {
        self.canvas.set_draw_color(SColor::RGB(0, 0, 0));
        self.canvas.clear();

        let mut index = 0;
        for y in 0..144 {
            for x in 0..160 {
                let color = pixels[x + y * 160];

                let color = match color {
                    Color::Off      => SColor::RGB(0x00, 0x00, 0x00),
                    Color::Dark     => SColor::RGB(0x55, 0x55, 0x55),
                    Color::Light    => SColor::RGB(0xab, 0xab, 0xab),
                    Color::On       => SColor::RGB(0xff, 0xff, 0xff),
                };

                self.canvas.set_draw_color(color);
                self.canvas.draw_point(Point::new(x as i32, y as i32));

                index += 1;
            }
        }

        self.canvas.present();
    }
}
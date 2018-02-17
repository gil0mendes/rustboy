pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
pub const SCREEN_EMPTY: ScreenBuffer = [Color::Off; SCREEN_PIXELS];

#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    Off = 0,
    Light = 1,
    Dark = 2,
    On = 3
}

impl Color {
    #[inline]
    pub fn from_u8(value: u8) -> Color {
        use self::Color::*;
        match value {
            1 => Light,
            2 => Dark,
            3 => On,
            _ => Off
        }
    }
}

pub type ScreenBuffer = [Color; SCREEN_PIXELS];
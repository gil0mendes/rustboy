use self::types::Color;

pub mod types;

const VOAM_SIZE: usize = 0xa0;

const ACCESS_OAM_CYCLES: isize = 21;
const CHARACTER_RAM_TILES: usize = 384;
const OAM_SPRITES: usize = 40;
const TILE_MAP_SIZE: usize = 0x400;
const UNDEFINED_READ: u8 = 0xff;

#[derive(Clone, Copy)]
struct Tile {
    data: [u8; 16]
}

impl Tile {
    fn new() -> Tile {
        Tile {
            data: [0; 16]
        }
    }
}

bitflags!(
    struct SpriteFlags: u8 {
        const UNUSED_MASK = 0b_0000_1111;
        const PALETTE     = 0b_0001_0000;
        const FLIPX       = 0b_0010_0000;
        const FLIPY       = 0b_0100_0000;
        const PRIORITY    = 0b_1000_0000;
    }
);

#[derive(Clone, Copy)]
struct Sprite {
    x: u8,
    y: u8,
    tile_num: u8,
    flags: SpriteFlags,
}

impl Sprite {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            tile_num: 0,
            flags: SpriteFlags::empty(),
        }
    }
}

bitflags!(
  struct Control: u8 {
    const BG_ON = 1 << 0;
    const OBJ_ON = 1 << 1;
    const OBJ_SIZE = 1 << 2;
    const BG_MAP = 1 << 3;
    const BG_ADDR = 1 << 4;
    const WINDOW_ON = 1 << 5;
    const WINDOW_MAP = 1 << 6;
    const LCD_ON = 1 << 7;
  }
);

bitflags!(
  struct Stat: u8 {
    const COMPARE = 1 << 2;
    const HBLANK_INT = 1 << 3;
    const VBLANK_INT = 1 << 4;
    const ACCESS_OAM_INT = 1 << 5;
    const COMPARE_INT = 1 << 6;
  }
);

struct Palette {
    off: Color,
    light: Color,
    dark: Color,
    on: Color,
    bits: u8,
}

impl Palette {
    fn new() -> Self {
        Palette {
            off: Color::On,
            light: Color::On,
            dark: Color::On,
            on: Color::On,
            bits: 0xff,
        }
    }

    fn set_bits(&mut self, value: u8) {
        self.off = Color::from_u8((value >> 0) & 0x3);
        self.light = Color::from_u8((value >> 2) & 0x3);
        self.dark = Color::from_u8((value >> 4) & 0x3);
        self.on = Color::from_u8((value >> 6) & 0x3);

        self.bits = value;
    }

    fn get(&self, color: &Color) -> Color {
        match *color {
            Color::Off => self.off,
            Color::Light => self.light,
            Color::Dark => self.dark,
            Color::On => self.on
        }
    }
}

#[derive(PartialEq, Eq)]
enum Mode {
    AccessOam,
    AccessVram,
    HBlank,
    VBlank,
}

pub struct Gpu {
    /// LCDC (LCD Control)
    control: Control,
    /// STATE (LCDC Status)
    status: Stat,
    /// Current line
    current_line: u8,
    /// Compare line
    compare_line: u8,
    /// Scroll X
    scroll_x: u8,
    /// Scroll Y
    scroll_y: u8,
    /// Window X Position
    window_x: u8,
    /// Window Y Position,
    window_y: u8,
    /// Background palette
    bg_palette: Palette,
    /// Object palette 0
    obj_palette0: Palette,
    /// Object palette 1
    obj_palette1: Palette,
    mode: Mode,
    cycles: isize,
    character_ram: [Tile; CHARACTER_RAM_TILES],
    oam: [Sprite; OAM_SPRITES],
    tile_map1: [u8; TILE_MAP_SIZE],
    tile_map2: [u8; TILE_MAP_SIZE],
    pub back_buffer: Box<types::ScreenBuffer>,
    vramBank: u8,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            control: Control::empty(),
            status: Stat::empty(),
            current_line: 0,
            compare_line: 0,
            scroll_x: 0,
            scroll_y: 0,
            window_x: 0,
            window_y: 0,
            bg_palette: Palette::new(),
            obj_palette0: Palette::new(),
            obj_palette1: Palette::new(),
            mode: Mode::AccessOam,
            cycles: ACCESS_OAM_CYCLES,
            character_ram: [Tile::new(); CHARACTER_RAM_TILES],
            oam: [Sprite::new(); OAM_SPRITES],
            tile_map1: [0; TILE_MAP_SIZE],
            tile_map2: [0; TILE_MAP_SIZE],
            back_buffer: Box::new(types::SCREEN_EMPTY),
            vramBank: 0,
        }
    }

    pub fn write_character_ram(&mut self, address: u16, value: u8) {
        if self.mode == Mode::AccessVram {
            return;
        }

        let tile = &mut self.character_ram[address as usize / 16];
        tile.data[address as usize % 16] = value;
    }

    pub fn read_character_ram(&self, address: u16) -> u8 {
        if self.mode == Mode::AccessVram {
            return UNDEFINED_READ;
        }

        let tile = &self.character_ram[address as usize / 16];
        tile.data[address as usize % 16]
    }

    pub fn write_tile_map1(&mut self, address: u16, value: u8) {
        if self.mode == Mode::AccessVram {
            return;
        }

        self.tile_map1[address as usize] = value;
    }

    pub fn read_tile_map1(&self, address: u16) -> u8 {
        if self.mode == Mode::AccessVram {
            return UNDEFINED_READ;
        }

        self.tile_map1[address as usize]
    }

    pub fn write_tile_map2(&mut self, address: u16, value: u8) {
        if self.mode == Mode::AccessVram {
            return;
        }

        self.tile_map2[address as usize] = value;
    }

    pub fn read_tile_map2(&self, address: u16) -> u8 {
        if self.mode == Mode::AccessVram {
            return UNDEFINED_READ;
        }

        self.tile_map2[address as usize]
    }

    pub fn set_bg_palette(&mut self, value: u8) {
        self.bg_palette.set_bits(value);
    }

    pub fn do_cycle(&mut self, trick: u32) {
        if !self.control.contains(Control::LCD_ON) {
            return;
        }
    }

    pub fn set_scroll_y(&mut self, value: u8) {
        self.scroll_y = value;
    }

    pub fn set_scroll_x(&mut self, value: u8) {
        self.scroll_x = value;
    }

    pub fn set_window_y(&mut self, value: u8) {
        self.window_y = value;
    }

    pub fn set_window_x(&mut self, value: u8) {
        self.window_x = value;
    }

    pub fn set_vramBank(&mut self, value: u8) {
        self.vramBank = value;
    }

    /// Read from the OAM memory
    pub fn read_oam(&self, address: u16) -> u8 {
        if self.mode == Mode::AccessVram || self.mode == Mode::AccessOam {
            return UNDEFINED_READ;
        }

        let sprite = &self.oam[address as usize / 4];

        match address as usize % 4 {
            3 => sprite.flags.bits(),
            2 => sprite.tile_num,
            1 => sprite.x.wrapping_add(8),
            _ => sprite.y.wrapping_add(16),
        }
    }
}

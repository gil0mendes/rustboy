const VOAM_SIZE: usize = 0xa0;

#[derive(Debug)]
pub struct Gpu {
    /// LCDC (LCD Control)
    control: u8,
    /// STATE (LCDC Status)
    status: u8,
    /// SCY (Scroll Y)
    scy: u8,
    /// SCX (Scroll X)
    scx: u8,
    // LY (LCDC Y-Coordinate)
    ly: u8,
    // LYC (LY Compare)
    lyc: u8,
    // WY (Window Y Position),
    wy: u8,
    // WX (Window X Position minus 7),
    wx: u8,
    // BGP (BG Pallet Data)
    bgp: u8,
    // OBP0
    obp0: u8,
    // OBP1
    obp1: u8,
    /// VRAM
    vram: Vec<u8>,
    /// OAM
    voam: Vec<u8>,
    /// Select VRAM bank
    vrambank: u8,
}

impl Gpu {
    /// create a new GPU instance
    pub fn new() -> Gpu {
        Gpu {
            voam: vec![0x20; VOAM_SIZE],
            vram: vec![0xca; 0x2000],
            control: 0,
            status: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            vrambank: 1
        }
    }

    /// Read a byte from the VRAM
    pub fn vram(&self, address: u16) -> u8 {
        self.vram[address as usize]
    }

    /// Write a byte to the VRAM
    pub fn set_vram(&mut self, address: u16, value: u8) {
        self.vram[address as usize] = value;
    }


    // --- OLD

    /// read one byte from the GPU memory area
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            // VOAM
            0xfe00 ... 0xfe99 => self.voam[address as usize & (VOAM_SIZE - 0x1)],
            // control
            0xff40 => self.control,
            // Status
            0xff41 => self.status,
            // SCY
            0xff42 => self.scy,
            // SCX
            0xff43 => self.scx,
            // LY
            0xff44 => self.ly,
            // LYC
            0xff45 => self.lyc,
            // BGP
            0xff47 => self.bgp,
            // OBP0
            0xff48 => self.obp0,
            // OBP1
            0xff49 => self.obp1,
            // WY
            0xff4a => self.wy,
            // WX
            0xff4b => self.wx,
            // Other addresses
            _ => panic!("GPU can read {:#x} address", address)
        }
    }

    /// write one byte to the GPU memory area
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            // VOAM
            0xfe00 ... 0xfe99 => self.voam[address as usize & (VOAM_SIZE - 0x1)] = value,
            // control
            0xff40 => self.control = value,
            // Status
            0xff41 => self.status = value,
            // SCY
            0xff42 => self.scy = value,
            // SCX
            0xff43 => self.scx = value,
            // LY - line (only read are supported)
            0xff44 => {}
            // LYC
            0xff45 => self.lyc = value,
            // BGP
            0xff47 => self.bgp = value,
            // OBP0
            0xff48 => self.obp0 = value,
            // OBP1
            0xff49 => self.obp1 = value,
            // WY
            0xff4a => self.wy = value,
            // WX
            0xff4b => self.wx = value,
            // Other addresses
            _ => panic!("GPU can't write on {:#x} address", address),
        }
    }
}
